.PHONY: default
default:
	@echo "Choose a Makefile target:"
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print "  - " $$1}}' | sort

.PHONY: check-deps
check-deps:
	@cargo make --help >/dev/null 2>&1 || { \
		echo >&2 "ERROR: Install cargo-make to use make tasks."; \
		echo >&2 "$ cargo install --no-default-features --force --locked cargo-make"; \
		echo >&2 "More info: https://sagiegurari.github.io/cargo-make"; \
		echo >&2; \
		exit 1; \
	}

.PHONY: setup
setup: check-deps
	cargo make setup

.PHONY: check
check: check-deps
	cargo make check

.PHONY: clean
clean: check-deps
	cargo make clean

# Build
.PHONY: quick
quick: check-deps
	cargo make quick

.PHONY: build
build: check-deps
	cargo make build

# Docker
.PHONY: docker
docker: quick
	cargo make docker

# Kubernetes create
.PHONY: k8s-infra-create
k8s-infra-create: quick
	cargo make k8s-infra-create

.PHONY: k8s-platform-create
k8s-platform-create: quick
	cargo make k8s-platform-create

.PHONY: k8s-app-create
k8s-app-create: quick
	cargo make k8s-app-create

# Kubernetes delete
.PHONY: k8s-app-delete
k8s-app-delete:
	cargo make k8s-app-delete

.PHONY: k8s-platform-delete
k8s-platform-delete:
	cargo make k8s-platform-delete

.PHONY: k8s-infra-delete
k8s-infra-delete:
	cargo make k8s-infra-delete

# Tests
.PHONY: test
test: quick
	cargo make test

.PHONY: integration-test
integration-test: quick
	cargo make integration-test
