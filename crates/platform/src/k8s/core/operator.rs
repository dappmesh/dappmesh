use futures::{StreamExt, TryFuture};
use kube::{
	api::{Patch, PatchParams},
	runtime::{controller::Action, watcher::Config, Controller},
	Api, Client, Resource, ResourceExt,
};
use kube_core::NamespaceResourceScope;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{error::Error, fmt::Debug, future::Future, hash::Hash, marker::PhantomData, sync::Arc};
use tokio::time::Duration;
use tracing::{error, info, instrument};
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

#[derive(PartialEq)]
pub enum OperatorAction {
	Create,
	Delete,
	NoOp,
}

pub trait OperatorController<Crd>
where
	Crd: Clone
		+ Resource<Scope = NamespaceResourceScope>
		+ DeserializeOwned
		+ Debug
		+ Send
		+ Sync
		+ 'static,
	Crd::DynamicType: Default + Eq + Hash + Clone + Debug + Unpin,
	Self: Sync + Send,
{
	#[instrument(skip(self, context), fields(trace_id))]
	fn reconcile(
		&self,
		resource: Arc<Crd>,
		context: Arc<OperatorContext>,
	) -> impl Future<Output = Result<Action, OperatorError>> + Send
	where
		Crd: OperatorResource,
	{
		async move {
			info!("Reconciling: {:?}", resource.name_any());
			match Self::action(Arc::clone(&resource)) {
				OperatorAction::Create => {
					info!("Creating resources for: {:?}", resource.name_any());
					self.handle_creation(&context.client, Arc::clone(&resource)).await?;
					Ok(Action::requeue(Duration::from_secs(5)))
				}
				OperatorAction::Delete => {
					info!("Deleting resources for: {:?}", resource.name_any());
					self.handle_deletion(&context.client, Arc::clone(&resource)).await?;
					Ok(Action::await_change())
				}
				OperatorAction::NoOp => {
					info!("No operation, state has not changed");
					Ok(Action::requeue(Duration::from_secs(10)))
				}
			}
		}
	}

	fn action(resource: Arc<Crd>) -> OperatorAction
	where
		Crd: OperatorResource,
	{
		if resource.should_delete() {
			OperatorAction::Delete
		} else if resource.should_create() {
			OperatorAction::Create
		} else {
			OperatorAction::NoOp
		}
	}

	fn handle_creation(
		&self,
		client: &Client,
		resource: Arc<Crd>,
	) -> impl Future<Output = Result<(), OperatorError>> + Send {
		async move {
			self.create_resources().await?;
			self.create_finalizer(&client, resource).await?;
			Ok(())
		}
	}

	fn handle_deletion(
		&self,
		client: &Client,
		resource: Arc<Crd>,
	) -> impl Future<Output = Result<(), OperatorError>> + Send {
		async move {
			self.delete_resources().await?;
			self.delete_finalizer(&client, resource).await?;
			Ok(())
		}
	}

	fn create_finalizer(
		&self,
		client: &Client,
		resource: Arc<Crd>,
	) -> impl Future<Output = Result<(), OperatorError>> + Send {
		async move {
			let finalizer: Value = json!({
				"metadata": {
					"finalizers": [self.finalizer()]
				}
			});
			self.patch(&client, resource, finalizer).await?;
			Ok(())
		}
	}

	fn delete_finalizer(
		&self,
		client: &Client,
		resource: Arc<Crd>,
	) -> impl Future<Output = Result<(), OperatorError>> + Send {
		async move {
			let finalizer: Value = json!({
				"metadata": {
					"finalizers": []
				}
			});
			self.patch(&client, resource, finalizer).await?;
			Ok(())
		}
	}

	fn patch(
		&self,
		client: &Client,
		resource: Arc<Crd>,
		finalizer: Value,
	) -> impl Future<Output = Result<(), OperatorError>> + Send {
		async move {
			if let Some(namespace) = resource.namespace() {
				let api: Api<Crd> = Api::namespaced(client.clone(), &namespace);
				let patch: Patch<&Value> = Patch::Merge(&finalizer);
				api.patch(&resource.name_any(), &PatchParams::default(), &patch).await?;
				Ok(())
			} else {
				Err(OperatorError::UserInputError(
					"Expected resource to be namespaced.".to_string(),
				))
			}
		}
	}

	fn create_resources(&self) -> impl Future<Output = Result<(), OperatorError>> + Send;
	fn delete_resources(&self) -> impl Future<Output = Result<(), OperatorError>> + Send;
	fn finalizer(&self) -> &str;
}

pub struct OperatorContext {
	pub client: Client,
}

impl OperatorContext {
	pub fn new(client: Client) -> Self {
		OperatorContext {
			client,
		}
	}
}

pub struct Operator<C>(PhantomData<C>);

impl<Crd> Operator<Crd>
where
	Crd: Clone + Resource + DeserializeOwned + Debug + Send + Sync + 'static,
	Crd::DynamicType: Default + Eq + Hash + Clone + Debug + Unpin,
{
	#[instrument(skip(_context))]
	fn on_error<ReconcileFut>(
		resource: Arc<Crd>,
		error: &ReconcileFut::Error,
		_context: Arc<OperatorContext>,
	) -> Action
	where
		ReconcileFut: TryFuture<Ok = Action> + Send + 'static,
		ReconcileFut::Error: Error + Send + 'static,
	{
		error!("Error while reconciling {:?}: {}", resource.name_any(), error.to_string());
		Action::requeue(Duration::from_secs(5))
	}

	pub async fn run<ReconcileFut>(
		reconcile: impl FnMut(Arc<Crd>, Arc<OperatorContext>) -> ReconcileFut,
	) where
		ReconcileFut: TryFuture<Ok = Action> + Send + 'static,
		ReconcileFut::Error: Error + Send + 'static,
	{
		let logger = tracing_subscriber::fmt::layer().compact();
		let env_filter = EnvFilter::try_from_default_env().or(EnvFilter::try_new("info")).unwrap();
		let collector = Registry::default().with(logger).with(env_filter);
		tracing::subscriber::set_global_default(collector).unwrap();

		let client = Client::try_default().await.expect("Failed to create a Kubernetes client.");
		let context: Arc<OperatorContext> = Arc::new(OperatorContext::new(client.clone()));
		let crd_api: Api<Crd> = Api::all(client.clone());
		let controller = Controller::new(crd_api, Config::default());

		controller
			.run(reconcile, Self::on_error::<ReconcileFut>, context)
			.for_each(|reconciliation_result| async move {
				match reconciliation_result {
					Ok(echo_resource) => {
						println!("Reconciliation successful. Resource: {:?}", echo_resource);
					}
					Err(reconciliation_err) => {
						eprintln!("Reconciliation error: {:?}", reconciliation_err)
					}
				}
			})
			.await;
	}
}

pub trait OperatorResource
where
	Self: Resource,
{
	fn should_delete(&self) -> bool {
		self.meta().deletion_timestamp.is_some()
	}
	fn should_create(&self) -> bool {
		self.meta().finalizers.as_ref().map_or(true, |finalizers| finalizers.is_empty())
	}
}

#[derive(Debug, thiserror::Error)]
pub enum OperatorError {
	#[error("Kubernetes reported error: {source}")]
	KubeError {
		#[from]
		source: kube::Error,
	},
	#[error("Invalid CRD: {0}")]
	UserInputError(String),
}
