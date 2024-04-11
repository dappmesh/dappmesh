use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dapp_platform::core::operator::OperatorResource;

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
