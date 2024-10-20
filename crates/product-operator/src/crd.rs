use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dapp_platform::k8s::core::operator::OperatorResource;

#[derive(
	CustomResource, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, JsonSchema,
)]
#[kube(
	doc = "DappMesh product controller custom resource.",
	group = "dappmesh.io",
	version = "v1alpha1",
	kind = "DappProduct",
	namespaced,
	singular = "dappproduct",
	plural = "dappproducts",
	shortname = "product",
	shortname = "products",
	derive = "PartialEq",
	derive = "Default"
)]
pub struct DappProductSpec {
	#[schemars(regex(pattern = r"[A-Za-z0-9](?:[A-Za-z0-9\-]{0,61}[A-Za-z0-9])?"))]
	name: String,
}

impl OperatorResource for DappProduct {}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
	use kube::{Resource, ResourceExt};

	impl DappProduct {
		pub fn test_instance() -> Self {
			let mut product = DappProduct::new("test_instance", DappProductSpec::default());
			product.meta_mut().namespace = Some("test_namespace".to_string());
			product
		}

		pub fn finalize(mut self, finalizer: String) -> Self {
			self.finalizers_mut().push(finalizer);
			self
		}

		pub fn remove_finalizer(mut self) -> Self {
			self.meta_mut().finalizers = None;
			self
		}

		pub fn deletion_timestamp(mut self) -> Self {
			self.meta_mut().deletion_timestamp = Some(Time(Utc::now()));
			self
		}
	}
}
