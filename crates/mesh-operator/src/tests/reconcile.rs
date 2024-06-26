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
		crd::{DappMesh, DappMeshSpec},
		operator::MeshOperatorController,
	};
	use dapp_platform::core::operator::{OperatorContext, OperatorController};

	const MESH_NAMESPACE: &str = "default";
	const MESH_NAME: &str = "mesh-test";

	#[tokio::test]
	#[ignore = "Integration Test, uses real k8s context"]
	async fn should_reconcile_resource_lifecycle() -> Result<(), Error> {
		let client = Client::try_default().await.unwrap();
		let mesh_api: Api<DappMesh> = Api::namespaced(client.clone(), MESH_NAMESPACE);
		let mesh_resource = DappMesh::new(MESH_NAME, DappMeshSpec::default());

		let controller = MeshOperatorController::new(
			MESH_NAME.to_string(),
			MESH_NAMESPACE.to_string(),
			client.clone(),
		);

		let context: Arc<OperatorContext> = Arc::new(OperatorContext::new(client.clone()));
		let mesh = mesh_api.create(&PostParams::default(), &mesh_resource).await?;

		let action = controller.reconcile(Arc::new(mesh), context.clone()).await?;
		ensure!(action == Action::requeue(Duration::from_secs(5)));

		let finalizers = mesh_api.get(MESH_NAME).await?.metadata.finalizers;
		ensure!(finalizers == Some(vec![MeshOperatorController::FINALIZER.to_string()]));

		let mesh = mesh_api.delete(MESH_NAME, &DeleteParams::default()).await?;

		let action = controller.reconcile(Arc::new(mesh.left_or_default()), context).await?;
		ensure!(action == Action::await_change());

		Ok(())
	}
}
