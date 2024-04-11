use kube::{runtime::controller::Action, ResourceExt};
use std::sync::Arc;

use dapp_mesh_operator::{crd::DappMesh, operator::MeshOperatorController};
use dapp_platform::core::operator::{Operator, OperatorContext, OperatorController, OperatorError};

#[tokio::main]
async fn main() {
	Operator::run(reconcile).await;
}

async fn reconcile(
	resource: Arc<DappMesh>,
	context: Arc<OperatorContext>,
) -> Result<Action, OperatorError> {
	if let Some(namespace) = resource.namespace() {
		let controller =
			MeshOperatorController::new(resource.name_any(), namespace, context.client.clone());
		controller.reconcile(resource, context).await
	} else {
		Err(OperatorError::UserInputError("Expected resource to be namespaced.".to_string()))
	}
}
