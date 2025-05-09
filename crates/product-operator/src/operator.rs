use kube::Client;

use crate::crd::DappProduct;
use dapp_core::k8s::operator::{OperatorController, OperatorError};
use dapp_core::k8s::storage::surrealdb::app::DappSurrealDB;

pub struct ProductOperatorController<'a> {
	pub surrealdb: DappSurrealDB<'a>,
}

impl<'a> ProductOperatorController<'a> {
	pub const FINALIZER: &'static str = "dappproducts.dappmesh.io/finalizer";

	pub fn new(name: String, namespace: String, client: &'a Client) -> Self {
		Self {
			surrealdb: DappSurrealDB::new(name, namespace, client),
		}
	}
}

impl OperatorController<DappProduct> for ProductOperatorController<'_> {
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
	use dapp_core::k8s::operator::OperatorAction;
	use std::sync::Arc;

	#[tokio::test]
	async fn determine_action_returns_requeue_if_finalizer_is_empty() {
		let product = DappProduct::test_instance();

		let action = ProductOperatorController::action(Arc::new(product));

		matches!(action, OperatorAction::Create);
	}

	#[tokio::test]
	async fn determine_action_returns_await_change_if_finalizer_is_empty() {
		let product = DappProduct::test_instance().deletion_timestamp();

		let action = ProductOperatorController::action(Arc::new(product));

		matches!(action, OperatorAction::Delete);
	}

	#[tokio::test]
	async fn determine_action_returns_requeue_if_election_timestamp_is_empty_but_finalizer_is_not()
	{
		let product =
			DappProduct::test_instance().finalize(ProductOperatorController::FINALIZER.to_string());

		let action = ProductOperatorController::action(Arc::new(product));

		matches!(action, OperatorAction::NoOp);
	}
}
