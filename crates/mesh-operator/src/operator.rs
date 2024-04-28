use kube::Client;

use dapp_platform::{
	core::operator::{OperatorController, OperatorError},
	surrealdb::app::SurrealDBApp,
};

use crate::crd::DappMesh;

pub struct MeshOperatorController {
	pub surrealdb_app: SurrealDBApp,
}

impl MeshOperatorController {
	pub const FINALIZER: &'static str = "dappmeshs.dappmesh.io/finalizer";

	pub fn new(name: String, namespace: String, client: Client) -> Self {
		Self {
			surrealdb_app: SurrealDBApp::new(name, namespace, client),
		}
	}
}

impl OperatorController<DappMesh> for MeshOperatorController {
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
		let mesh = DappMesh::test_instance();

		let action = MeshOperatorController::action(Arc::new(mesh));

		matches!(action, OperatorAction::Create);
	}

	#[tokio::test]
	async fn determine_action_returns_await_change_if_finalizer_is_empty() {
		let mesh = DappMesh::test_instance().deletion_timestamp();

		let action = MeshOperatorController::action(Arc::new(mesh));

		matches!(action, OperatorAction::Delete);
	}

	#[tokio::test]
	async fn determine_action_returns_requeue_if_eletion_timestamp_is_empty_but_finalizer_is_not() {
		let mesh =
			DappMesh::test_instance().finalize(MeshOperatorController::FINALIZER.to_string());

		let action = MeshOperatorController::action(Arc::new(mesh));

		matches!(action, OperatorAction::NoOp);
	}
}
