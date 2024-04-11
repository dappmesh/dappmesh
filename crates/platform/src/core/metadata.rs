use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct MetadataOptions {
	pub name: String,
	pub namespace: String,
	pub labels: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MetadataConfig {
	pub options: MetadataOptions,
}

impl MetadataConfig {
	pub fn new(name: String, namespace: String, labels: BTreeMap<String, String>) -> Self {
		Self {
			options: MetadataOptions {
				name: name.to_owned(),
				namespace: namespace.to_owned(),
				labels,
			},
		}
	}

	pub fn metadata(&self) -> ObjectMeta {
		ObjectMeta {
			name: Some(self.options.name.to_owned()),
			namespace: Some(self.options.namespace.to_owned()),
			labels: Some(self.options.labels.to_owned()),
			..ObjectMeta::default()
		}
	}
}
