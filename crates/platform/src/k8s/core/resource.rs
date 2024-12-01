use k8s_openapi::{
	api::{
		apps::v1::StatefulSet,
		core::v1::{PersistentVolumeClaim, Service, ServiceAccount},
	},
	apimachinery::pkg::apis::meta::v1::ObjectMeta,
	Metadata, NamespaceResourceScope,
};
use kube::{
	api::{ListParams, PostParams},
	Api, Client, Error as KubeError,
};
use kube_core::{params::DeleteParams, Resource, ResourceExt};
use regex::Regex;
use serde::{de::DeserializeOwned, de::Error, Serialize};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

static LABEL_SELECTOR: &str = "app.kubernetes.io/part-of";

#[macro_export]
macro_rules! include_resource {
	($path:expr) => {{
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $path))
	}};
}

pub struct DappResource<T> {
	content: String,
	_phantom: PhantomData<T>,
}

impl<T> DappResource<T>
where
	T: Resource<Scope = NamespaceResourceScope>
		+ ResourceExt
		+ Metadata<Ty = ObjectMeta>
		+ Clone
		+ Debug
		+ DeserializeOwned
		+ Serialize,
	<T as Resource>::DynamicType: Default,
{
	pub fn new(yaml_str: &str, values: &HashMap<String, String>) -> Self {
		Self {
			content: Self::replace(yaml_str.to_string(), values),
			_phantom: PhantomData,
		}
	}

	pub async fn create(
		&self,
		client: &Client,
		name: &str,
		namespace: &str,
	) -> Result<(), KubeError> {
		let api: Api<T> = Api::namespaced(client.clone(), namespace);
		let params = self.label_selector(name);
		let resource: T = serde_yaml::from_str(&self.content).map_err(|e| {
			let json_err = serde_json::Error::custom(e.to_string());
			KubeError::SerdeError(json_err)
		})?;

		if api.list(&params).await?.items.is_empty() {
			api.create(&PostParams::default(), &resource).await?;
		}
		Ok(())
	}

	pub async fn delete(
		&self,
		client: &Client,
		name: &str,
		namespace: &str,
	) -> Result<(), KubeError> {
		let api: Api<T> = Api::namespaced(client.clone(), namespace);
		let params = self.label_selector(name);

		for resource in api.list(&params).await? {
			api.delete(&resource.name_any(), &DeleteParams::default()).await?;
		}
		Ok(())
	}

	fn label_selector(&self, name: &str) -> ListParams {
		let selector = format!("{}={}", LABEL_SELECTOR, name);
		ListParams::default().labels(&selector)
	}

	fn replace(template: String, values: &HashMap<String, String>) -> String {
		let re = Regex::new(r"\{\{\s(?P<key>[\w.]+)\s}}").unwrap();
		re.replace_all(&template, |caps: &regex::Captures| {
			let key = &caps["key"];
			values.get(key).cloned().unwrap_or_else(|| caps[0].to_string())
		})
		.to_string()
	}
}

pub type DappPersistenceVolumeClaim = DappResource<PersistentVolumeClaim>;
pub type DappService = DappResource<Service>;
pub type DappServiceAccount = DappResource<ServiceAccount>;
pub type DappStatefulSet = DappResource<StatefulSet>;
