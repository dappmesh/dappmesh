use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dapp_platform::core::operator::OperatorResource;

#[derive(
	CustomResource, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, JsonSchema,
)]
#[kube(
	doc = "DappMesh controller custom resource.",
	group = "dappmesh.io",
	version = "v1alpha1",
	kind = "DappMesh",
	namespaced,
	singular = "dappmesh",
	plural = "dappmeshs",
	shortname = "mesh",
	shortname = "meshs",
	derive = "PartialEq",
	derive = "Default"
)]
pub struct DappMeshSpec {
	#[schemars(regex(pattern = r"[A-Za-z0-9](?:[A-Za-z0-9\-]{0,61}[A-Za-z0-9])?"))]
	name: String,
}

impl OperatorResource for DappMesh {}
