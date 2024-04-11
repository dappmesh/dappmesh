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
	const FINALIZER: &'static str = "dappmeshs.dappmesh.io/finalizer";

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
