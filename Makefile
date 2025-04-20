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
	cargo make --profile development setup

.PHONY: check-dev
check-dev: check-deps
	cargo make --profile development check-dev

.PHONY: check-release
check-release: check-deps
	cargo make --profile release check-relase

.PHONY: clean
clean: check-deps
	cargo make clean

.PHONY: build-dev
build-dev: check-deps
	cargo make --profile development build-dev

.PHONY: build-release
build-release: check-deps
	cargo make --profile release build-release

# Docker
.PHONY: docker-dev
docker-dev: build-dev
	cargo make --profile development docker-dev

.PHONY: docker-release
docker-release: build-release
	cargo make --profile release docker-release

.PHONY: test
test: build-dev
	cargo make --profile development test

.PHONY: integration-test
integration-test: build-dev
	cargo make --profile development integration-test
