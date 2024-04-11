use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

#[derive(Clone)]
pub struct SurrealDBConfig {
	pub name: String,
	pub namespace: String,
	pub service: SurrealDBServiceConfig,
	pub statefulset: SurrealDBStatefulSetConfig,
}

#[derive(Clone)]
pub struct SurrealDBServiceConfig {
	pub protocol: String,
	pub port: i32,
}

#[derive(Clone)]
pub struct SurrealDBStatefulSetConfig {
	pub replicas: i32,
	pub container: ContainerConfig,
	pub pvc: PVCConfig,
}

#[derive(Clone)]
pub struct ContainerConfig {
	pub image: String,
	pub path: String,
	pub user: String,
	pub password: String,
	pub log_level: String,
	pub port: i32,
	pub port_name: String,
	pub protocol: String,
}

#[derive(Clone)]
pub struct PVCConfig {
	pub access_modes: Vec<String>,
	pub resource_quantity: Quantity,
}

impl SurrealDBConfig {
	pub fn new(name: String, namespace: String) -> Self {
		Self {
			name,
			namespace,
			service: SurrealDBServiceConfig::default(),
			statefulset: SurrealDBStatefulSetConfig::default(),
		}
	}
}

impl SurrealDBServiceConfig {
	fn default() -> Self {
		Self {
			protocol: "TCP".to_string(),
			port: 8080,
		}
	}
}

impl SurrealDBStatefulSetConfig {
	fn default() -> Self {
		Self {
			replicas: 1,
			container: ContainerConfig::default(),
			pvc: PVCConfig::default(),
		}
	}
}

impl ContainerConfig {
	fn default() -> Self {
		Self {
			image: "surrealdb/surrealdb:v1.3.0".to_string(),
			path: "/data/store".to_string(),
			user: "root".to_string(),
			password: "root".to_string(),
			log_level: "info".to_string(),
			port: 8080,
			port_name: "http".to_string(),
			protocol: "TCP".to_string(),
		}
	}
}

impl PVCConfig {
	fn default() -> Self {
		Self {
			access_modes: vec!["ReadWriteOnce".to_string()],
			resource_quantity: Quantity("1Gi".to_string()),
		}
	}
}
