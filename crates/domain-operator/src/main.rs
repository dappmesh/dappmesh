use kube::{runtime::controller::Action, ResourceExt};
use std::sync::Arc;

use dapp_domain_operator::{crd::DappDomain, operator::DomainOperatorController};
use dapp_platform::core::operator::{Operator, OperatorContext, OperatorController, OperatorError};

#[tokio::main]
async fn main() {
	Operator::run(reconcile).await;
}

async fn reconcile(
	resource: Arc<DappDomain>,
	context: Arc<OperatorContext>,
) -> Result<Action, OperatorError> {
	if let Some(namespace) = resource.namespace() {
		let controller =
			DomainOperatorController::new(resource.name_any(), namespace, context.client.clone());
		controller.reconcile(resource, context).await
	} else {
		Err(OperatorError::UserInputError("Expected resource to be namespaced.".to_string()))
	}
}
