use k8s_openapi::api::{
	apps::v1::StatefulSet,
	core::v1::{PersistentVolumeClaim, Service, ServiceAccount},
};

use kube::{
	api::{DeleteParams, ListParams, PostParams},
	Api, Client, Error, ResourceExt,
};

use super::{
	config::SurrealDBConfig,
	service::SurrealDBService,
	service_account::SurrealDBServiceAccount,
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
