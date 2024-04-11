# Roadmap

- Create Kubernetes Application MVP
- Keep Documentation in Sync with Development

## Backlog

- Create [add and remove finalizer](https://github.com/Pscheidl/rust-kubernetes-operator-example/blob/master/src/finalizer.rs) logic for domain resource
- Look into [testing strategy](https://kube.rs/controllers/testing/)
- Look into doing real deployment to colima cluster
- Look into [validating](https://docs.rs/kube/latest/kube/derive.CustomResource.html#schema-validation) dynamic crd rust objects
- Look into deploying surrealdb on domain resource creation
- Look into [tracing_subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) for logging