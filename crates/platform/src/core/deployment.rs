use k8s_openapi::{
	api::{
		apps::v1::{Deployment, DeploymentSpec, DeploymentStatus},
		core::v1::{Container, PodSpec, PodTemplateSpec},
	},
	apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta},
};

use super::labels::Labels;

pub struct NoMetaData;

#[derive(Clone)]
pub struct NoSpec;

pub struct DeploymentBuilder<Meta, Spec> {
	pub metadata: Meta,
	pub spec: Spec,
}

impl Default for DeploymentBuilder<NoMetaData, NoSpec> {
	fn default() -> Self {
		Self {
			metadata: NoMetaData,
			spec: NoSpec,
		}
	}
}

impl<Spec: Clone> DeploymentBuilder<NoMetaData, Spec> {
	pub fn metadata(
		&self,
		name: &str,
		owner: &str,
		namespace: &str,
	) -> DeploymentBuilder<ObjectMeta, Spec> {
		let labels = Labels::labels(vec![
			("component", "database"),
			("version", "latest"),
			("part-of", owner),
			("name", name),
		]);

		let metadata = ObjectMeta {
			name: Some(name.to_string()),
			namespace: Some(namespace.to_string()),
			labels: Some(labels),
			..ObjectMeta::default()
		};

		DeploymentBuilder {
			metadata,
			spec: self.spec.clone(),
		}
	}
}

impl<Spec> DeploymentBuilder<ObjectMeta, Spec> {
	pub fn spec(
		&self,
		replicas: i32,
		containers: Vec<Container>,
	) -> DeploymentBuilder<ObjectMeta, DeploymentSpec> {
		let stateful_set_spec = DeploymentSpec {
			replicas: Some(replicas),
			selector: LabelSelector {
				match_expressions: None,
				match_labels: self.metadata.labels.clone(),
			},
			template: PodTemplateSpec {
				spec: Some(PodSpec {
					containers,
					..PodSpec::default()
				}),
				metadata: Some(self.metadata.clone()),
			},
			..DeploymentSpec::default()
		};

		DeploymentBuilder {
			metadata: self.metadata.clone(),
			spec: stateful_set_spec,
		}
	}
}

impl DeploymentBuilder<ObjectMeta, DeploymentSpec> {
	pub fn manifest(&self) -> Deployment {
		Deployment {
			metadata: self.metadata.clone(),
			spec: Some(self.spec.clone()),
			status: Some(DeploymentStatus::default()),
		}
	}
}
