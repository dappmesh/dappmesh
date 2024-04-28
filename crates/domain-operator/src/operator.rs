use kube::Client;

use dapp_platform::{
	core::operator::{OperatorController, OperatorError},
	surrealdb::app::SurrealDBApp,
};

use crate::crd::DappDomain;

pub struct DomainOperatorController {
	pub surrealdb_app: SurrealDBApp,
}

impl DomainOperatorController {
	pub const FINALIZER: &'static str = "dappdomains.dappmesh.io/finalizer";

	pub fn new(name: String, namespace: String, client: Client) -> Self {
		Self {
			surrealdb_app: SurrealDBApp::new(name, namespace, client),
		}
	}
}

impl OperatorController<DappDomain> for DomainOperatorController {
	async fn create_resources(&self) -> Result<(), OperatorError> {
		self.surrealdb_app.create().await?;
		Ok(())
	}

	async fn delete_resources(&self) -> Result<(), OperatorError> {
		self.surrealdb_app.delete().await?;
		Ok(())
	}

	fn finalizer(&self) -> &str {
		Self::FINALIZER
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use dapp_platform::core::operator::OperatorAction;
	use std::sync::Arc;

	#[tokio::test]
	async fn determine_action_returns_requeue_if_finalizer_is_empty() {
		let domain = DappDomain::test_instance();

		let action = DomainOperatorController::action(Arc::new(domain));

		matches!(action, OperatorAction::Create);
	}

	#[tokio::test]
	async fn determine_action_returns_await_change_if_finalizer_is_empty() {
		let domain = DappDomain::test_instance().deletion_timestamp();

		let action = DomainOperatorController::action(Arc::new(domain));

		matches!(action, OperatorAction::Delete);
	}

	#[tokio::test]
	async fn determine_action_returns_requeue_if_eletion_timestamp_is_empty_but_finalizer_is_not() {
		let domain =
			DappDomain::test_instance().finalize(DomainOperatorController::FINALIZER.to_string());

		let action = DomainOperatorController::action(Arc::new(domain));

		matches!(action, OperatorAction::NoOp);
	}
}
