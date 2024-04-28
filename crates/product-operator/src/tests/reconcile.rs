#[cfg(test)]
mod tests {
	use anyhow::{ensure, Error};
	use kube::{
		api::{DeleteParams, PostParams},
		runtime::controller::Action,
		Api, Client,
	};
	use std::sync::Arc;
	use tokio::time::Duration;

	use crate::{
		crd::{DappProduct, DappProductSpec},
		operator::ProductOperatorController,
	};
	use dapp_platform::core::operator::{OperatorContext, OperatorController};

	const PRODUCT_NAMESPACE: &str = "default";
	const PRODUCT_NAME: &str = "product-test";

	#[tokio::test]
	#[ignore = "Integration Test, uses real k8s context"]
	async fn should_reconcile_resource_lifecycle() -> Result<(), Error> {
		let client = Client::try_default().await.unwrap();
		let product_api: Api<DappProduct> = Api::namespaced(client.clone(), PRODUCT_NAMESPACE);
		let product_resource = DappProduct::new(PRODUCT_NAME, DappProductSpec::default());

		let controller = ProductOperatorController::new(
			PRODUCT_NAME.to_string(),
			PRODUCT_NAMESPACE.to_string(),
			client.clone(),
		);

		let context: Arc<OperatorContext> = Arc::new(OperatorContext::new(client.clone()));
		let product = product_api.create(&PostParams::default(), &product_resource).await?;

		let action = controller.reconcile(Arc::new(product), context.clone()).await?;
		ensure!(action == Action::requeue(Duration::from_secs(5)));

		let finalizers = product_api.get(PRODUCT_NAME).await?.metadata.finalizers;
		ensure!(finalizers == Some(vec![ProductOperatorController::FINALIZER.to_string()]));

		let product = product_api.delete(PRODUCT_NAME, &DeleteParams::default()).await?;

		let action = controller.reconcile(Arc::new(product.left_or_default()), context).await?;
		ensure!(action == Action::await_change());

		Ok(())
	}
}
