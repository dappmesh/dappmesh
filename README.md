# Rust Data Mesh Framework

**DappMesh** is a framework for creating cloud-native data mesh distributed applications declaratively.

At its core, DappMesh embraces the principles of Data Mesh by **decentralizing data ownership** and management across **domain-oriented data product** applications.
This is achieved through a distributed architecture that implements Kubernetes Operator Framework to manage a specific Data Mesh component as a custom resource.
In addition, each **data product** is a distributed system that contains multiple operators managing storage, processing, **federated governance**, and **self-service data endpoints**.

This framework is designed for organizations looking to implement Data Mesh in a Kubernetes environment, providing a scalable, flexible solution to meet the complex demands of modern data management.

## Vision

- Energy Efficient Data Platform.
- High-quality Data. Its not a data swamp.
- Rust-powered.
- Cloud Native.
- Infrastructure agnostic.

## Architecture

- [Composite Operator Mesh](docs/architecture/operator-mesh.md)
- [Data Product](docs/architecture/data-product.md)
- [Data Application Model](docs/architecture/application-model.md)

## Components

- [Mesh Operator](docs/project/mesh-operator.md)
- [Domain Operator](docs/project/domain-operator.md)
- [Product Operator](docs/project/product-operator.md)
- [Platform](docs/project/platform.md)

## Using DappMesh

### Requirements

- Rust: [rustup](https://rustup.rs/)
- Docker: [Docker](https://www.docker.com/products/docker-desktop), [Podman](https://podman.io/getting-started/installation), [Moby](https://mobyproject.org/), [Colima](https://github.com/abiosoft/colima), etc.
- Docker [buildx plugin](https://github.com/docker/buildx).
- Kubernetes distribution: [kind](https://kind.sigs.k8s.io/), [minikube](https://minikube.sigs.k8s.io/docs/), [k3d](https://k3d.io/), [k3s](https://k3s.io/), [microk8s](https://microk8s.io/), etc.
- Kubernetes client: [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)

### Cargo Plugins

Install cargo-make:
```console
cargo install --force cargo-make
```

Using the dependency checking tools locally requires installing the following software:
```console
cargo install --locked cargo-deny
cargo install --locked cargo-vet

# Linux
cargo install --locked cargo-acl
sudo apt install -y bubblewrap # Adapt as required
```

### OS Specific Setup

- [Linux](./docs/env/linux.md)
- [MacOS](./docs/env/macos.md)
- [Windows](./docs/env/windows.md)

### Setup Access to GitHub Container Registry

Read the section [Authenticating with a personal access token (classic)](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-with-a-personal-access-token-classic). 

Follow the steps 1, 2, and 3 to set up your local docker to push images into `https://ghcr.io` container registry.

### Build & Deploy

```console
# check format and dependencies
make check

# quick build
make quick

# build and push docker images
make docker

# full build
make build
```
For more commands, read the [Makefile](./Makefile) and [Makefile.toml](./Makefile.toml)

### Kubernetes Deployment

To create the Data Mesh crds run:
```console
kubectl apply -f manifest/crds
```

To create the Data Mesh application run: 

```
kubectl apply -f manifest/app
```

### Local Tests

#### Unit tests

To run unit tests run:

```
cargo make test
```

#### Integration tests
These tests require that a k8s instance is running and your machine has the credentials and configuration needed to manage resources on the cluster. To run these tests execute the following command:

```
cargo make integration-test
```

#### Quick Dev Testing
If you want to quickly test your changes against a real cluster while developing and you dont want to push a new image to ghcr for the operator pod, you can use [mirrord](https://mirrord.dev/) to run your binary in a k8s cluster. First you must deploy an operator pod to the cluster:

```
kubectl apply -f manifest/app/{resource}-operator.yaml
```

Then run the following command to run your current changes against the cluster configuration:

```
cargo build && mirrord exec -t deploy/{name of your operator deployment} --steal ./target/debug/{bin}
```

## Contributing

:triangular_flag_on_post: Read the [How to Contribute](./CONTRIBUTING.md) section before modifying the code.

## Resources

[Operator Reference](https://github.com/Pscheidl/rust-kubernetes-operator-example/tree/master)
