#[cfg(test)]
mod tests {
	use anyhow::{ensure, Error};
	use dapp_domain_operator::crd::{DappDomain, DappDomainSpec};
	use dapp_domain_operator::operator::DomainOperatorController;
	use dapp_platform::k8s::core::operator::{OperatorContext, OperatorController};
	use kube::{
		api::{DeleteParams, PostParams},
		runtime::controller::Action,
		Api, Client,
	};
	use std::sync::Arc;
	use tokio::time::Duration;

	const DOMAIN_NAMESPACE: &str = "default";
	const DOMAIN_NAME: &str = "domain-test";

	#[tokio::test]
	#[ignore = "Integration Test, uses real k8s context"]
	async fn should_reconcile_resource_lifecycle() -> Result<(), Error> {
		let client = Client::try_default().await?;
		let domain_api: Api<DappDomain> = Api::namespaced(client.clone(), DOMAIN_NAMESPACE);
		let domain_resource = DappDomain::new(DOMAIN_NAME, DappDomainSpec::default());

		let operator_client = client.clone();
		let operator = DomainOperatorController::new(
			DOMAIN_NAME.to_string(),
			DOMAIN_NAMESPACE.to_string(),
			&operator_client,
		);

		let context: Arc<OperatorContext> = Arc::new(OperatorContext::new(client.clone()));
		let domain = domain_api.create(&PostParams::default(), &domain_resource).await?;

		let action = operator.reconcile(Arc::new(domain), context.clone()).await?;
		ensure!(action == Action::requeue(Duration::from_secs(5)));

		let finalizers = domain_api.get(DOMAIN_NAME).await?.metadata.finalizers;
		ensure!(finalizers == Some(vec![DomainOperatorController::FINALIZER.to_string()]));

		let domain = domain_api.delete(DOMAIN_NAME, &DeleteParams::default()).await?;

		let action = operator.reconcile(Arc::new(domain.left_or_default()), context).await?;
		ensure!(action == Action::await_change());

		Ok(())
	}
}
