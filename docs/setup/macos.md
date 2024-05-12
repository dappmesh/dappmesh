# Setup Local Development Environment on MacOS

## Mise Setup

[Mise](https://mise.jdx.dev/getting-started.html) configuration is stored in .config/mise/.config.local.toml.

To install tools, run the following command:

```console
mise install
```
To start a local Kubernetes instance and container runtime, run the following command:
```console
colima start --cpu 3 --memory 5 --runtime containerd --kubernetes
```