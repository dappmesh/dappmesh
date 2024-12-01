use kube::Client;

use crate::crd::DappDomain;
use dapp_platform::k8s::core::operator::{OperatorController, OperatorError};
use dapp_platform::k8s::storage::surrealdb::app::DappSurrealDB;

pub struct DomainOperatorController<'a> {
	pub surrealdb: DappSurrealDB<'a>,
}

impl<'a> DomainOperatorController<'a> {
	pub const FINALIZER: &'static str = "dappdomains.dappmesh.io/finalizer";

	pub fn new(name: String, namespace: String, client: &'a Client) -> Self {
		Self {
			surrealdb: DappSurrealDB::new(name, namespace, client),
		}
	}
}

impl<'a> OperatorController<DappDomain> for DomainOperatorController<'a> {
	async fn create_resources(&self) -> Result<(), OperatorError> {
		self.surrealdb.create().await?;
		Ok(())
	}

	async fn delete_resources(&self) -> Result<(), OperatorError> {
		self.surrealdb.delete().await?;
		Ok(())
	}

	fn finalizer(&self) -> &str {
		Self::FINALIZER
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use dapp_platform::k8s::core::operator::OperatorAction;
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
