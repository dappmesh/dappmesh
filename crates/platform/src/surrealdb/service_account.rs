use k8s_openapi::api::core::v1::ServiceAccount;

use super::config::SurrealDBConfig;
use crate::core::service_account::ServiceAccountBuilder;

pub struct SurrealDBServiceAccount {
	pub name: String,
	pub namespace: String,
}

impl SurrealDBServiceAccount {
	pub fn new(config: SurrealDBConfig) -> Self {
		Self {
			name: config.name,
			namespace: config.namespace,
		}
	}

	pub fn manifest(&self) -> ServiceAccount {
		ServiceAccountBuilder::new(&self.name, &self.namespace).manifest()
	}

	pub fn get_labels(&self) -> String {
		format!("part-of={}", self.name)
	}
}
