use k8s_openapi::{api::core::v1::ServiceAccount, apimachinery::pkg::apis::meta::v1::ObjectMeta};

use super::labels::Labels;

pub struct ServiceAccountBuilder {
	pub metadata: ObjectMeta,
}

impl ServiceAccountBuilder {
	pub fn new(name: &str, namespace: &str) -> Self {
		let labels = Labels::labels(vec![
			("component", "database"),
			("version", "latest"),
			("part-of", name),
		]);

		let obj_metadata = ObjectMeta {
			name: Some(name.to_owned()),
			namespace: Some(namespace.to_owned()),
			labels: Some(labels.to_owned()),
			..ObjectMeta::default()
		};

		Self {
			metadata: obj_metadata,
		}
	}

	pub fn manifest(&self) -> ServiceAccount {
		ServiceAccount {
			metadata: self.metadata.clone(),
			..ServiceAccount::default()
		}
	}
}
