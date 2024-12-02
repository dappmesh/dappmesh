use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dapp_platform::k8s::core::operator::OperatorResource;

#[derive(
	CustomResource, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, JsonSchema,
)]
#[kube(
	doc = "DappMesh mesh controller custom resource.",
	group = "dappmesh.io",
	version = "v1alpha1",
	kind = "DappMesh",
	namespaced,
	singular = "dappmesh",
	plural = "dappmeshes",
	shortname = "mesh",
	shortname = "meshes",
	derive = "PartialEq",
	derive = "Default"
)]
pub struct DappMeshSpec {
	#[schemars(regex(pattern = r"[A-Za-z0-9](?:[A-Za-z0-9\-]{0,61}[A-Za-z0-9])?"))]
	name: String,
}

impl OperatorResource for DappMesh {}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Utc;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
	use kube::{Resource, ResourceExt};

	impl DappMesh {
		pub fn test_instance() -> Self {
			let mut mesh = DappMesh::new("test_instance", DappMeshSpec::default());
			mesh.meta_mut().namespace = Some("test_namespace".to_string());
			mesh
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
