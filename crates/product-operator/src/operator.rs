use kube::Client;

use dapp_platform::{
	core::operator::{OperatorController, OperatorError},
	surrealdb::app::SurrealDBApp,
};

use crate::crd::DappProduct;

pub struct ProductOperatorController {
	pub surrealdb_app: SurrealDBApp,
}

impl ProductOperatorController {
	pub const FINALIZER: &'static str = "dappproducts.dappmesh.io/finalizer";

	pub fn new(name: String, namespace: String, client: Client) -> Self {
		Self {
			surrealdb_app: SurrealDBApp::new(name, namespace, client),
		}
	}
}

impl OperatorController<DappProduct> for ProductOperatorController {
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
	async fn determine_action_returns_requeue_if_eletion_timestamp_is_empty_but_finalizer_is_not() {
		let product =
			DappProduct::test_instance().finalize(ProductOperatorController::FINALIZER.to_string());

		let action = ProductOperatorController::action(Arc::new(product));

		matches!(action, OperatorAction::NoOp);
	}
}
