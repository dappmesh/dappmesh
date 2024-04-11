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

.PHONY: quick
quick: check-deps
	cargo make quick

.PHONY: docker
docker: quick
	cargo make docker

.PHONY: build
build: check-deps
	cargo make build
