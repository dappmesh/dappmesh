use k8s_openapi::{
    api::{
        apps::v1::StatefulSet,
        core::v1::{Container, ContainerPort, EnvVar, HTTPGetAction, Probe},
    },
    apimachinery::pkg::util::intstr::IntOrString,
};
use k8s_openapi::api::core::v1::VolumeMount;

use super::config::{SurrealDBConfig, SurrealDBStatefulSetConfig};
use crate::core::statefulset::StatefulSetBuilder;

pub struct SurrealDBStatefulSet {
    pub name: String,
    pub namespace: String,
    pub config: SurrealDBStatefulSetConfig,
}

impl SurrealDBStatefulSet {
    pub fn new(config: SurrealDBConfig) -> Self {
        Self {
            name: config.name,
            namespace: config.namespace,
            config: config.statefulset,
        }
    }

    fn containers(&self) -> Vec<Container> {
        let mut containers: Vec<Container> = Vec::new();
        let config = &self.config.container;

        let container = Container {
            name: self.name.clone(),
            image: Some(config.image.clone()),
            image_pull_policy: Some("IfNotPresent".to_string()),
            args: Some(vec!["start".to_owned()]),
            env: Some(self.container_env()),
            ports: Some(vec![ContainerPort {
                container_port: config.port,
                name: Some(config.port_name.clone()),
                ..ContainerPort::default()
            }]),
            liveness_probe: Some(self.http_probe(config.port)),
            readiness_probe: Some(self.http_probe(config.port)),
            volume_mounts: Some(vec![VolumeMount {
                name: self.name.to_string(),
                mount_path: config.path.clone(),
                ..VolumeMount::default()
            }]),
            ..Container::default()
        };

        containers.push(container);
        containers
    }

    fn container_env(&self) -> Vec<EnvVar> {
        let config = &self.config.container;

        vec![
            EnvVar {
                name: "SURREAL_PATH".to_owned(),
                value: Some(format!("file:{}", config.path.clone())),
                value_from: None,
            },
            EnvVar {
                name: "SURREAL_NO_BANNER".to_owned(),
                value: Some("true".to_owned()),
                value_from: None,
            },
            EnvVar {
                name: "SURREAL_LOG".to_owned(),
                value: Some("info".to_owned()),
                value_from: None,
            },
            EnvVar {
                name: "SURREAL_BIND".to_owned(),
                value: Some(format!("0.0.0.0:{}", config.port)),
                value_from: None,
            },
            EnvVar {
                name: "SURREAL_USER".to_owned(),
                value: Some(config.user.clone()),
                value_from: None,
            },
            EnvVar {
                name: "SURREAL_PASS".to_owned(),
                value: Some(config.password.clone()),
                value_from: None,
            },
        ]
    }

    fn http_probe(&self, port: i32) -> Probe {
        Probe {
            http_get: Some(HTTPGetAction {
                path: Some("/health".to_string()),
                port: IntOrString::Int(port),
                ..HTTPGetAction::default()
            }),
            ..Probe::default()
        }
    }

    pub fn manifest(&self) -> StatefulSet {
        StatefulSetBuilder::default()
            .metadata(&self.name, &self.name, &self.namespace)
            .spec(
                self.config.replicas,
                self.containers(),
                self.config.pvc.access_modes.clone(),
                self.config.pvc.resource_quantity.clone(),
            )
            .manifest()
    }

    pub fn get_labels(&self) -> String {
        format!("part-of={}", self.name)
    }
}