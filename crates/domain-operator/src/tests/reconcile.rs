#[cfg(test)]
mod tests {
	use anyhow::{ensure, Error};
	use kube::{
		api::{DeleteParams, PostParams},
		runtime::controller::Action,
		Api, Client,
	};
	use std::sync::Arc;
	use tokio::time::Duration;

	use crate::{
		crd::{DappDomain, DappDomainSpec},
		operator::DomainOperatorController,
	};
	use dapp_platform::core::operator::{OperatorContext, OperatorController};

	const DOMAIN_NAMESPACE: &str = "default";
	const DOMAIN_NAME: &str = "domain-test";

	#[tokio::test]
	#[ignore = "Integration Test, uses real k8s context"]
	async fn should_reconcile_resource_lifecycle() -> Result<(), Error> {
		let client = Client::try_default().await.unwrap();
		let domain_api: Api<DappDomain> = Api::namespaced(client.clone(), DOMAIN_NAMESPACE);
		let domain_resource = DappDomain::new(DOMAIN_NAME, DappDomainSpec::default());

		let controller = DomainOperatorController::new(
			DOMAIN_NAME.to_string(),
			DOMAIN_NAMESPACE.to_string(),
			client.clone(),
		);

		let context: Arc<OperatorContext> = Arc::new(OperatorContext::new(client.clone()));
		let domain = domain_api.create(&PostParams::default(), &domain_resource).await?;

		let action = controller.reconcile(Arc::new(domain), context.clone()).await?;
		ensure!(action == Action::requeue(Duration::from_secs(5)));

		let finalizers = domain_api.get(DOMAIN_NAME).await?.metadata.finalizers;
		ensure!(finalizers == Some(vec![DomainOperatorController::FINALIZER.to_string()]));

		let domain = domain_api.delete(DOMAIN_NAME, &DeleteParams::default()).await?;

		let action = controller.reconcile(Arc::new(domain.left_or_default()), context).await?;
		ensure!(action == Action::await_change());

		Ok(())
	}
}
