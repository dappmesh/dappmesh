# Setup Development Environment on Linux

Download and install [homebrew](https://brew.sh/). Make sure that there is an [env variable](https://docs.brew.sh/Homebrew-on-Linux#install). 

## Mise Setup

[Mise](https://mise.jdx.dev/getting-started.html) configuration is stored in .config/mise/.config.local.toml.

To install tools, run the following command:
```console
mise install
```

To start a local Kubernetes instance, run the following command:
```console
colima start --cpu 3 --memory 5 --runtime containerd --kubernetes
```