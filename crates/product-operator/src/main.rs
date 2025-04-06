use kube::{runtime::controller::Action, ResourceExt};
use std::sync::Arc;

use dapp_core::k8s::operator::{Operator, OperatorContext, OperatorController, OperatorError};
use dapp_product_operator::{crd::DappProduct, operator::ProductOperatorController};

#[tokio::main]
async fn main() {
	Operator::run(reconcile).await;
}

async fn reconcile(
	resource: Arc<DappProduct>,
	context: Arc<OperatorContext>,
) -> Result<Action, OperatorError> {
	if let Some(namespace) = resource.namespace() {
		let controller =
			ProductOperatorController::new(resource.name_any(), namespace, &context.client);
		controller.reconcile(resource, Arc::clone(&context)).await
	} else {
		Err(OperatorError::UserInputError("Expected resource to be namespaced.".to_string()))
	}
}
