use std::collections::HashMap;

pub struct SurrealDBConfig {
	pub name: String,
	pub namespace: String,
	pub values: HashMap<String, String>,
}

impl SurrealDBConfig {
	pub fn new(name: String, namespace: String) -> Self {
		let mut values: HashMap<String, String> = HashMap::new();
		values.insert("name".to_string(), name.clone());
		values.insert("namespace".to_string(), namespace.clone());
		values.insert("part.of".to_string(), name.clone());
		values.insert("password".to_string(), "dappmesh".to_string());
		values.insert("port".to_string(), "8080".to_string());
		values.insert("replicas".to_string(), "1".to_string());
		values.insert("storage.quantity".to_string(), "2GB".to_string());
		values.insert("user".to_string(), "dappmesh".to_string());

		Self {
			name,
			namespace,
			values,
		}
	}
}