use k8s_openapi::api::{
	apps::v1::StatefulSet,
	core::v1::{PersistentVolumeClaim, Service, ServiceAccount},
};

use kube::{
	api::{DeleteParams, ListParams, PostParams},
	Api, Client, Error, ResourceExt,
};

use super::{
	config::SurrealDBConfig, service::SurrealDBService, service_account::SurrealDBServiceAccount,
	statefulset::SurrealDBStatefulSet,
};

pub struct SurrealDBApp {
	client: Client,
	pub service: SurrealDBService,
	pub service_account: SurrealDBServiceAccount,
	pub statefulset: SurrealDBStatefulSet,
}

impl SurrealDBApp {
	pub fn new(name: String, namespace: String, client: Client) -> Self {
		let config = SurrealDBConfig::new(format!("{}-db", name), namespace);

		Self {
			client,
			service: SurrealDBService::new(config.clone()),
			service_account: SurrealDBServiceAccount::new(config.clone()),
			statefulset: SurrealDBStatefulSet::new(config.clone()),
		}
	}

	pub async fn create(&self) -> Result<(), Error> {
		self.create_service_account().await?;
		self.create_statefulset().await?;
		self.create_service().await?;
		Ok(())
	}

	async fn create_service(&self) -> Result<(), Error> {
		let api: Api<Service> = Api::namespaced(self.client.clone(), &self.service.namespace);
		let params = ListParams::default().labels(&self.service.get_labels());

		if api.list(&params).await?.items.is_empty() {
			api.create(&PostParams::default(), &self.service.manifest()).await?;
		}

		Ok(())
	}

	async fn create_service_account(&self) -> Result<(), Error> {
		let api: Api<ServiceAccount> =
			Api::namespaced(self.client.clone(), &self.service_account.namespace);
		let params = ListParams::default().labels(&self.service_account.get_labels());

		if api.list(&params).await?.items.is_empty() {
			api.create(&PostParams::default(), &self.service_account.manifest()).await?;
		}

		Ok(())
	}

	pub async fn create_statefulset(&self) -> Result<(), Error> {
		let api: Api<StatefulSet> =
			Api::namespaced(self.client.clone(), &self.statefulset.namespace);
		let lp = ListParams::default().labels(&self.statefulset.get_labels());

		if api.list(&lp).await?.items.is_empty() {
			api.create(&PostParams::default(), &self.statefulset.manifest()).await?;
		}

		Ok(())
	}

	pub async fn delete(&self) -> Result<(), Error> {
		self.delete_service().await?;
		self.delete_statefulset().await?;
		self.delete_pvc().await?;
		self.delete_service_account().await?;
		Ok(())
	}

	pub async fn delete_pvc(&self) -> Result<(), Error> {
		let api: Api<PersistentVolumeClaim> =
			Api::namespaced(self.client.clone(), &self.statefulset.namespace);
		let lp = ListParams::default().labels(&self.statefulset.get_labels());

		for pvc in api.list(&lp).await? {
			api.delete(&pvc.name_any(), &DeleteParams::default()).await?;
		}

		Ok(())
	}

	pub async fn delete_service(&self) -> Result<(), Error> {
		let api: Api<Service> = Api::namespaced(self.client.clone(), &self.service.namespace);
		let params = ListParams::default().labels(&self.service.get_labels());

		for service in api.list(&params).await? {
			api.delete(&service.name_any(), &DeleteParams::default()).await?;
		}
		Ok(())
	}

	pub async fn delete_service_account(&self) -> Result<(), Error> {
		let api: Api<ServiceAccount> =
			Api::namespaced(self.client.clone(), &self.service_account.namespace);

		let params = ListParams::default().labels(&self.service_account.get_labels());

		for sa in api.list(&params).await? {
			api.delete(&sa.name_any(), &DeleteParams::default()).await?;
		}

		Ok(())
	}

	pub async fn delete_statefulset(&self) -> Result<(), Error> {
		let api: Api<StatefulSet> =
			Api::namespaced(self.client.clone(), &self.statefulset.namespace);

		let lp = ListParams::default().labels(&self.statefulset.get_labels());

		for ss in api.list(&lp).await? {
			api.delete(&ss.name_any(), &DeleteParams::default()).await?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hyper::{Request, Response, StatusCode};
	use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
	use kube::{
		api::{ListMeta, ObjectList, TypeMeta},
		client::Body,
		Error,
	};
	use serde_json::to_vec;
	use tokio::spawn;

	const TEST_NAME: &str = "test-name";
	const TEST_NAME_PREFIXED: &str = "test-name-db";
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
					"/api/v1/namespaces/{}/persistentvolumeclaims?&labelSelector=part-of%3D{}",
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
					"/api/v1/namespaces/{}/services?&labelSelector=part-of%3D{}",
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
					"/api/v1/namespaces/{}/serviceaccounts?&labelSelector=part-of%3D{}",
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
					"/apis/apps/v1/namespaces/{}/statefulsets?&labelSelector=part-of%3D{}",
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
			SurrealDBApp::new(TEST_NAME.to_string(), TEST_NAMESPACE.to_string(), client);

		mock_api_server.handle_create();
		let result = surreal_db_app.create().await;

		assert!(matches!(result, Ok(())));
	}

	#[tokio::test]
	async fn surreal_db_delete_resources() {
		let (client, mock_kube_api) = mock_client();
		let surreal_db_app =
			SurrealDBApp::new(TEST_NAME.to_string(), TEST_NAMESPACE.to_string(), client);

		mock_kube_api.handle_delete();
		let result = surreal_db_app.delete().await;
		assert!(matches!(result, Ok(())));
	}
}
