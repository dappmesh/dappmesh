# Energy Efficient Data Platform

**DappMesh** is a framework for creating cloud-native data applications declaratively.

At its core, DappMesh embraces the principles of Data Mesh by **decentralizing data ownership** and management across **domain-oriented data product** applications.
This is achieved through a distributed architecture that implements Kubernetes Operator Framework to manage a specific Data Mesh component as a custom resource.
In addition, each **data product** is a distributed system that contains multiple operators managing storage, processing, **federated governance**, and **self-service data endpoints**.

This framework is designed for organizations looking to implement Data Mesh in a Kubernetes environment, providing a scalable, flexible solution to meet the complex demands of modern data management.

## Vision

- **Energy Efficient Data Platform:** This requirement may seem opinionated and somewhat controversial, but we can establish some premises to achieve this goal.
  1. A programming language or computing/storage system is considered **eligible** when it fulfills all functional requirements of a specific feature.
  2. Whenever choosing between 2 **eligible** programming languages or computing/storage systems, we select the option that combines performance and resource efficiency to become the **standard**.
  3. A **standard** can be replaced by an **eligible** that outperforms it.

NOTE: You might want to check some concepts discussed in this AWS article about [Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust).


- **High-quality Data:** DappMesh is not a data swamp. According to the [Data Application Model](./docs/architecture/application-model.md), any data at rest, data in motion, or data in use must have a well-defined schema or metadata.


- **Cloud Native:** Just as Kubernetes YAML and Helm charts serve as the blueprint for orchestrating distributed systems, DappMesh enables the structured design and deployment of data products.
  In short, the desired state of a distributed data product application materializes through DappMesh.


- **Infrastructure Agnostic:** This flexibility ensures that DappMesh can seamlessly integrate with various underlying platforms.


- **Rust-powered:** [Why Rust is the most admired language among developers](https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/) :green_heart:

## Architecture

- [Composite Operator Mesh](docs/architecture/operator-mesh.md)
- [Data Product](docs/architecture/data-product.md)
- [Data Application Model](docs/architecture/application-model.md)

## Using DappMesh

### Requirements

- **Rust:** [rustup](https://rustup.rs/)
- **Docker:** [Docker](https://www.docker.com/products/docker-desktop), [Podman](https://podman.io/getting-started/installation), [Moby](https://mobyproject.org/), [Colima](https://github.com/abiosoft/colima), etc.
- **Docker Plugins:** [buildx plugin](https://github.com/docker/buildx).
- **Kubernetes distribution:** [kind](https://kind.sigs.k8s.io/), [minikube](https://minikube.sigs.k8s.io/docs/), [k3d](https://k3d.io/), [k3s](https://k3s.io/), [microk8s](https://microk8s.io/), etc.
- **Kubernetes client:** [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
- **DevSpace:** [devspace](https://www.devspace.sh/docs/getting-started/installation)

### OS Specific Setup

- [Linux](docs/setup/linux.md)
- [MacOS](docs/setup/macos.md)
- [Windows](docs/setup/windows.md)

### Cargo Plugins

Install cargo-make:
```shell
cargo install --no-default-features --force --locked cargo-make
```

Using the dependency checking tools locally requires installing the following software:
```shell
# ensure all dependencies conform to expectations and requirements
cargo install --locked cargo-deny

# ensure that third-party Rust dependencies have been audited by a trusted entity
cargo install --locked cargo-vet

# code ACL checker
cargo install --locked cargo-acl

# Linux: bubblewrap allows build scripts (build.rs), tests and rustc to be run inside a sandbox
sudo apt install -y bubblewrap # Adapt as required
```

### Setup Access to GitHub Container Registry

1. Create a GitHub personal access token (classic) in the user interface with this url:

```shell
# Select the scope: read:packages
# Select the scope: write:packages
# Select the scope: delete:packages
https://github.com/settings/tokens/new?scopes=write:packages
```

2. Save and export your personal access token (classic)

```shell
export GHCR_DAPPMESH_USER=YOUR_GITHUB_USER
export GHCR_DAPPMESH_TOKEN=YOUR_TOKEN
```

3. Using the CLI for your container type, sign in to the Container registry service:

```shell
$ echo $GHCR_DAPPMESH_TOKEN | docker login ghcr.io -u GHCR_DAPPMESH_USER --password-stdin
> Login Succeeded
```

4. Read the docs to troubleshoot: [Authenticating with a personal access token (classic)](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-with-a-personal-access-token-classic).

### Build & Test

```console
# check workspace, format, and dependencies
make check

# quick build
make quick

# unit tests
make test

# integration tests
make integration tests

# full build
make build

# build and push docker images
make docker

# clean build target
make clean
```
For more commands, read the [Makefile](./Makefile) and [Makefile.toml](./Makefile.toml)

### Local Kubernetes Deployment

1. To create the DappMesh resources:

:warning: **Warning:** The development container uses a rust debian image which can take a long time to load on the first try. Run ```devspace``` again in case of timeout.

```shell
# quick build 
make quick

# creates CRDs, infrastructure, core operators and data product application
devspace dev
```

2. To delete the DappMesh resources: 

```shell
# deletes the deployed kubernetes resources
devspace purge

# forces to purge even though it might be in use by another DevSpace project
devspace purge --force-purge
```

## Contributing

:triangular_flag_on_post: Read the [How to Contribute](./CONTRIBUTING.md) section before modifying the code.
