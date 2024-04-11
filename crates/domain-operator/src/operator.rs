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
	const FINALIZER: &'static str = "dappdomains.dappmesh.io/finalizer";

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
