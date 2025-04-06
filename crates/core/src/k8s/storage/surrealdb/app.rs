use super::config::SurrealDBConfig;
use crate::{
	include_resource,
	k8s::resource::{DappPersistenceVolumeClaim, DappService, DappServiceAccount, DappStatefulSet},
};
use kube::{Client, Error};

static SURREALDB_PVC_YAML: &str = include_resource!("storage/surrealdb/pvc.yaml");
static SURREALDB_SERVICE_YAML: &str = include_resource!("storage/surrealdb/service.yaml");
static SURREALDB_SERVICE_ACCOUNT_YAML: &str =
	include_resource!("storage/surrealdb/service-account.yaml");
static SURREALDB_STATEFULSET_YAML: &str = include_resource!("storage/surrealdb/statefulset.yaml");

pub struct DappSurrealDB<'a> {
	client: &'a Client,
	config: SurrealDBConfig,
	pvc: DappPersistenceVolumeClaim,
	service: DappService,
	service_account: DappServiceAccount,
	statefulset: DappStatefulSet,
}

impl<'a> DappSurrealDB<'a> {
	pub fn new(name: String, namespace: String, client: &'a Client) -> Self {
		let config = SurrealDBConfig::new(format!("{}-catalog", name), namespace);
		let values = &config.values;

		Self {
			client,
			pvc: DappPersistenceVolumeClaim::new(SURREALDB_PVC_YAML, values),
			service: DappService::new(SURREALDB_SERVICE_YAML, values),
			service_account: DappServiceAccount::new(SURREALDB_SERVICE_ACCOUNT_YAML, values),
			statefulset: DappStatefulSet::new(SURREALDB_STATEFULSET_YAML, values),
			config,
		}
	}

	pub async fn create(&self) -> Result<(), Error> {
		let name = &self.config.name;
		let namespace = &self.config.namespace;

		self.service_account.create(self.client, name, namespace).await?;
		self.statefulset.create(self.client, name, namespace).await?;
		self.service.create(self.client, name, namespace).await?;

		Ok(())
	}

	pub async fn delete(&self) -> Result<(), Error> {
		let name = &self.config.name;
		let namespace = &self.config.namespace;

		self.service.delete(self.client, name, namespace).await?;
		self.statefulset.delete(self.client, name, namespace).await?;
		self.pvc.delete(self.client, name, namespace).await?;
		self.service_account.delete(self.client, name, namespace).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hyper::{Request, Response, StatusCode};
	use k8s_openapi::{
		api::{
			apps::v1::StatefulSet,
			core::v1::PersistentVolumeClaim,
			core::v1::{Service, ServiceAccount},
		},
		apimachinery::pkg::apis::meta::v1::ObjectMeta,
	};
	use kube::{
		api::{ListMeta, ObjectList, TypeMeta},
		client::Body,
		Error,
	};
	use serde_json::to_vec;
	use tokio::spawn;

	const TEST_NAME: &str = "test-name";
	const TEST_NAME_PREFIXED: &str = "test-name-catalog";
	const TEST_NAMESPACE: &str = "test-namespace";

	type MockedKubeApiServerHandle = tower_test::mock::Handle<Request<Body>, Response<Body>>;

	pub struct MockedKubeApiServer(MockedKubeApiServerHandle);

	pub fn mock_client() -> (Client, MockedKubeApiServer) {
		let (mocked_service, handle) = tower_test::mock::pair();

		let mock_client = Client::new(mocked_service, "default");

		(mock_client, MockedKubeApiServer(handle))
	}

	fn mock_response(data: Vec<u8>) -> Response<Body> {
		Response::builder().status(StatusCode::OK).body(Body::from(data)).unwrap()
	}

	impl MockedKubeApiServer {
		pub fn handle_create(self) -> tokio::task::JoinHandle<MockedKubeApiServer> {
			spawn(async move {
				self.handle_list_service_accounts(true)
					.await
					.unwrap()
					.handle_create_service_account()
					.await
					.unwrap()
					.handle_list_statefulset(true)
					.await
					.unwrap()
					.handle_create_statefulset()
					.await
					.unwrap()
					.handle_list_service(true)
					.await
					.unwrap()
					.handle_create_service()
					.await
					.unwrap()
			})
		}

		pub async fn handle_create_service(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::POST);
			assert_eq!(
				request.uri().to_string(),
				format!("/api/v1/namespaces/{}/services?", TEST_NAMESPACE)
			);

			let response = to_vec(&Service::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);

			Ok(self)
		}

		pub async fn handle_create_service_account(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::POST);
			assert_eq!(
				request.uri().to_string(),
				format!("/api/v1/namespaces/{}/serviceaccounts?", TEST_NAMESPACE)
			);

			let response = to_vec(&ServiceAccount::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);

			Ok(self)
		}

		pub async fn handle_create_statefulset(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::POST);
			assert_eq!(
				request.uri().to_string(),
				format!("/apis/apps/v1/namespaces/{}/statefulsets?", TEST_NAMESPACE)
			);

			let response = to_vec(&StatefulSet::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);

			Ok(self)
		}

		pub fn handle_delete(self) -> tokio::task::JoinHandle<MockedKubeApiServer> {
			spawn(async move {
				self.handle_list_service(false)
					.await
					.unwrap()
					.handle_delete_service()
					.await
					.unwrap()
					.handle_list_statefulset(false)
					.await
					.unwrap()
					.handle_delete_statefulset()
					.await
					.unwrap()
					.handle_list_pvc(false)
					.await
					.unwrap()
					.handle_delete_pvc()
					.await
					.unwrap()
					.handle_list_service_accounts(false)
					.await
					.unwrap()
					.handle_delete_service_account()
					.await
					.unwrap()
			})
		}

		pub async fn handle_delete_pvc(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::DELETE);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/api/v1/namespaces/{}/persistentvolumeclaims/{}?",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let response = to_vec(&PersistentVolumeClaim::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_delete_service(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::DELETE);
			assert_eq!(
				request.uri().to_string(),
				format!("/api/v1/namespaces/{}/services/{}?", TEST_NAMESPACE, TEST_NAME_PREFIXED)
			);

			let response = to_vec(&Service::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_delete_service_account(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::DELETE);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/api/v1/namespaces/{}/serviceaccounts/{}?",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let response = to_vec(&ServiceAccount::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_delete_statefulset(mut self) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::DELETE);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/apis/apps/v1/namespaces/{}/statefulsets/{}?",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let response = to_vec(&StatefulSet::default()).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_list_pvc(mut self, creating: bool) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::GET);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/api/v1/namespaces/{}/persistentvolumeclaims?&labelSelector=app.kubernetes.io%2Fpart-of%3D{}",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let items: Vec<PersistentVolumeClaim> = if creating {
				vec![]
			} else {
				let pvc = PersistentVolumeClaim {
					metadata: ObjectMeta {
						name: Some(TEST_NAME_PREFIXED.to_string()),
						namespace: Some(TEST_NAMESPACE.to_string()),
						..ObjectMeta::default()
					},
					..PersistentVolumeClaim::default()
				};
				vec![pvc]
			};

			let list = ObjectList {
				types: TypeMeta::default(),
				metadata: ListMeta::default(),
				items,
			};

			let response = to_vec(&list).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_list_service(mut self, creating: bool) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::GET);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/api/v1/namespaces/{}/services?&labelSelector=app.kubernetes.io%2Fpart-of%3D{}",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let items: Vec<Service> = if creating {
				vec![]
			} else {
				let service = Service {
					metadata: ObjectMeta {
						name: Some(TEST_NAME_PREFIXED.to_string()),
						namespace: Some(TEST_NAMESPACE.to_string()),
						..ObjectMeta::default()
					},
					..Service::default()
				};
				vec![service]
			};

			let list = ObjectList {
				types: TypeMeta::default(),
				metadata: ListMeta::default(),
				items,
			};

			let response = to_vec(&list).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_list_service_accounts(mut self, creating: bool) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::GET);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/api/v1/namespaces/{}/serviceaccounts?&labelSelector=app.kubernetes.io%2Fpart-of%3D{}",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let items: Vec<ServiceAccount> = if creating {
				vec![]
			} else {
				let service_account = ServiceAccount {
					metadata: ObjectMeta {
						name: Some(TEST_NAME_PREFIXED.to_string()),
						namespace: Some(TEST_NAMESPACE.to_string()),
						..ObjectMeta::default()
					},
					..ServiceAccount::default()
				};
				vec![service_account]
			};

			let list = ObjectList {
				types: TypeMeta::default(),
				metadata: ListMeta::default(),
				items,
			};

			let response = to_vec(&list).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}

		pub async fn handle_list_statefulset(mut self, creating: bool) -> Result<Self, Error> {
			let (request, send) = self.0.next_request().await.expect("Service not called");
			assert_eq!(request.method(), &hyper::Method::GET);
			assert_eq!(
				request.uri().to_string(),
				format!(
					"/apis/apps/v1/namespaces/{}/statefulsets?&labelSelector=app.kubernetes.io%2Fpart-of%3D{}",
					TEST_NAMESPACE, TEST_NAME_PREFIXED
				)
			);

			let items: Vec<StatefulSet> = if creating {
				vec![]
			} else {
				let statefulset = StatefulSet {
					metadata: ObjectMeta {
						name: Some(TEST_NAME_PREFIXED.to_string()),
						namespace: Some(TEST_NAMESPACE.to_string()),
						..ObjectMeta::default()
					},
					..StatefulSet::default()
				};
				vec![statefulset]
			};

			let list = ObjectList {
				types: TypeMeta::default(),
				metadata: ListMeta::default(),
				items,
			};

			let response = to_vec(&list).unwrap();
			let mock_response = mock_response(response);
			send.send_response(mock_response);
			Ok(self)
		}
	}

	#[tokio::test]
	async fn surreal_db_create_resources() {
		let (client, mock_api_server) = mock_client();
		let surreal_db_app =
			DappSurrealDB::new(TEST_NAME.to_string(), TEST_NAMESPACE.to_string(), &client);

		mock_api_server.handle_create();
		let result = surreal_db_app.create().await;

		assert!(matches!(result, Ok(())));
	}

	#[tokio::test]
	async fn surreal_db_delete_resources() {
		let (client, mock_kube_api) = mock_client();
		let surreal_db_app =
			DappSurrealDB::new(TEST_NAME.to_string(), TEST_NAMESPACE.to_string(), &client);

		mock_kube_api.handle_delete();
		let result = surreal_db_app.delete().await;
		assert!(matches!(result, Ok(())));
	}
}
