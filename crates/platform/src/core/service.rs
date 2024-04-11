use k8s_openapi::{
	api::core::v1::{Service, ServicePort, ServiceSpec, ServiceStatus},
	apimachinery::pkg::{apis::meta::v1::ObjectMeta, util::intstr::IntOrString},
};

use super::labels::Labels;

pub struct NoMetaData;

#[derive(Clone)]
pub struct NoSpec;

pub struct ServiceBuilder<Meta, Spec> {
	pub metadata: Meta,
	pub spec: Spec,
}

impl Default for ServiceBuilder<NoMetaData, NoSpec> {
	fn default() -> Self {
		Self {
			metadata: NoMetaData,
			spec: NoSpec,
		}
	}
}

impl<Spec: Clone> ServiceBuilder<NoMetaData, Spec> {
	pub fn metadata(&self, name: &str, namespace: &str) -> ServiceBuilder<ObjectMeta, Spec> {
		let labels = Labels::labels(vec![
			("component", "networking"),
			("version", "latest"),
			("part-of", name),
		]);

		let obj_metadata = ObjectMeta {
			name: Some(name.to_string()),
			namespace: Some(namespace.to_string()),
			labels: Some(labels),
			..ObjectMeta::default()
		};

		ServiceBuilder {
			metadata: obj_metadata,
			spec: self.spec.clone(),
		}
	}
}

impl<Spec> ServiceBuilder<ObjectMeta, Spec> {
	pub fn service_spec(
		&self,
		protocol: &str,
		port: i32,
	) -> ServiceBuilder<ObjectMeta, ServiceSpec> {
		let spec = ServiceSpec {
			ports: Some(vec![ServicePort {
				protocol: Some(protocol.to_string()),
				port,
				target_port: Some(IntOrString::Int(port)),
				..ServicePort::default()
			}]),
			..ServiceSpec::default()
		};

		ServiceBuilder {
			metadata: self.metadata.clone(),
			spec,
		}
	}
}

impl ServiceBuilder<ObjectMeta, ServiceSpec> {
	pub fn manifest(&self) -> Service {
		Service {
			metadata: self.metadata.clone(),
			spec: Some(self.spec.clone()),
			status: Some(ServiceStatus::default()),
		}
	}
}
