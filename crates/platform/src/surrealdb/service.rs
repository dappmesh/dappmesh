use k8s_openapi::api::core::v1::Service;

use super::config::{SurrealDBConfig, SurrealDBServiceConfig};
use crate::core::service::ServiceBuilder;
pub struct SurrealDBService {
	pub name: String,
	pub namespace: String,
	config: SurrealDBServiceConfig,
}

impl SurrealDBService {
	pub fn new(config: SurrealDBConfig) -> Self {
		Self {
			name: config.name,
			namespace: config.namespace,
			config: config.service,
		}
	}

	pub fn manifest(&self) -> Service {
		ServiceBuilder::default()
			.metadata(&self.name, &self.namespace)
			.service_spec(&self.config.protocol, self.config.port)
			.manifest()
	}

	pub fn get_labels(&self) -> String {
		format!("part-of={}", self.name)
	}
}
