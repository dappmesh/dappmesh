use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dapp_platform::k8s::core::operator::OperatorResource;

#[derive(
	CustomResource, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, JsonSchema,
)]
#[kube(
	doc = "DappMesh domain controller custom resource.",
	group = "dappmesh.io",
	version = "v1alpha1",
	kind = "DappDomain",
	namespaced,
	singular = "dappdomain",
	plural = "dappdomains",
	shortname = "domain",
	shortname = "domains",
	derive = "PartialEq",
	derive = "Default"
)]
pub struct DappDomainSpec {
	#[schemars(regex(pattern = r"[A-Za-z0-9](?:[A-Za-z0-9\-]{0,61}[A-Za-z0-9])?"))]
	name: String,
}

impl OperatorResource for DappDomain {}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
	use kube::{Resource, ResourceExt};

	impl DappDomain {
		pub fn test_instance() -> Self {
			let mut domain = DappDomain::new("test_instance", DappDomainSpec::default());
			domain.meta_mut().namespace = Some("test_namespace".to_string());
			domain
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
