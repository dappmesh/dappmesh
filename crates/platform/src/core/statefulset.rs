use k8s_openapi::{
    api::{
        apps::v1::{StatefulSet, StatefulSetSpec, StatefulSetStatus},
        core::v1::{
            Container, PersistentVolumeClaim, PersistentVolumeClaimSpec, PodSpec, PodTemplateSpec,
            ResourceRequirements,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity,
        apis::meta::v1::{LabelSelector, ObjectMeta},
    },
};

use std::collections::BTreeMap;

use super::labels::Labels;

pub struct NoMetaData;

#[derive(Clone)]
pub struct NoSpec;

pub struct StatefulSetBuilder<Meta, Spec> {
    pub metadata: Meta,
    pub spec: Spec,
}

impl Default for StatefulSetBuilder<NoMetaData, NoSpec> {
    fn default() -> Self {
        Self {
            metadata: NoMetaData,
            spec: NoSpec,
        }
    }
}

impl<Spec: Clone> StatefulSetBuilder<NoMetaData, Spec> {
    pub fn metadata(
        &self,
        name: &str,
        owner: &str,
        namespace: &str,
    ) -> StatefulSetBuilder<ObjectMeta, Spec> {
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

        StatefulSetBuilder {
            metadata,
            spec: self.spec.clone(),
        }
    }
}

impl<Spec> StatefulSetBuilder<ObjectMeta, Spec> {
    pub fn spec(
        &self,
        replicas: i32,
        containers: Vec<Container>,
        access_modes: Vec<String>,
        resource_quantity: Quantity,
    ) -> StatefulSetBuilder<ObjectMeta, StatefulSetSpec> {
        let mut pvc_resources: BTreeMap<String, Quantity> = BTreeMap::new();
        pvc_resources.insert("storage".to_owned(), resource_quantity);

        let spec = StatefulSetSpec {
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
            volume_claim_templates: Some(vec![PersistentVolumeClaim {
                metadata: self.metadata.clone(),
                spec: Some(PersistentVolumeClaimSpec {
                    access_modes: Some(access_modes),
                    resources: Some(ResourceRequirements {
                        requests: Some(pvc_resources),
                        ..ResourceRequirements::default()
                    }),
                    ..PersistentVolumeClaimSpec::default()
                }),
                ..PersistentVolumeClaim::default()
            }]),
            ..StatefulSetSpec::default()
        };

        StatefulSetBuilder {
            metadata: self.metadata.clone(),
            spec,
        }
    }
}

impl StatefulSetBuilder<ObjectMeta, StatefulSetSpec> {
    pub fn manifest(&self) -> StatefulSet {
        StatefulSet {
            metadata: self.metadata.clone(),
            spec: Some(self.spec.clone()),
            status: Some(StatefulSetStatus::default()),
        }
    }
}
