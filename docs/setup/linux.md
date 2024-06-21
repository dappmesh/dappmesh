# Setup Development Environment on Linux

## Mise Setup

[Mise](https://mise.jdx.dev/getting-started.html) configuration is stored in .config/mise/.config.local.toml.

To install tools, run the following command:
```console
mise install
```

To start a local Kubernetes instance, run the following command:
```console
colima start --cpu 4 --memory 8 --runtime containerd --kubernetes
```