# TAFlow build and development targets.
#
#   make install   release wheel installed into the active environment
#   make dev       editable debug build + dev tooling, for working on taflow
#   make build     compile the native extension (.so) in place
#   make check     verify the Python implementation is correct
#   make bench     benchmark against TA-Lib
#
# `make help` lists everything.

PYTHON        ?= python3
MANIFEST      := crates/taflow-python/Cargo.toml
# Local measurement only. Distributed wheels rely on multiversion runtime
# dispatch instead, so target-cpu is never baked into a release build.
NATIVE_FLAGS  := -C target-cpu=native

.DEFAULT_GOAL := help
.PHONY: help install dev build build-native wheel check test test-rust \
        test-python verify verify-external bench docs fmt lint clean

help: ## Show this help
	@grep -hE '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) \
	  | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

# ---------------------------------------------------------------- build ----

install: ## Build and install the release wheel into the active environment
	$(PYTHON) -m pip install --upgrade pip maturin
	maturin build --release --manifest-path $(MANIFEST) --out dist
	$(PYTHON) -m pip install --force-reinstall --find-links dist taflow

dev: ## Editable debug build plus development dependencies
	$(PYTHON) -m pip install --upgrade pip maturin
	maturin develop --manifest-path $(MANIFEST)
	$(PYTHON) -m pip install numpy pytest

build: ## Compile the native extension (.so) into python/taflow/, optimized
	maturin develop --release --manifest-path $(MANIFEST)

build-native: ## Same as build but tuned to this CPU — benchmarking only, never ship
	RUSTFLAGS="$(NATIVE_FLAGS)" maturin develop --release --manifest-path $(MANIFEST)

wheel: ## Produce a distributable wheel in dist/
	maturin build --release --manifest-path $(MANIFEST) --out dist

# ------------------------------------------------------------ correctness ----

check: ## Verify the Python implementation: unit tests (incl. doc examples) + oracle parity
	uv run pytest -q
	uv run python scripts/verification/correctness.py $(ARGS)

test: test-rust test-python ## Run the Rust and Python unit test suites

test-rust: ## Rust unit and integration tests
	cargo test --workspace

test-python: ## Python unit tests (pipelines, adapters, API surface)
	uv run pytest -q

verify: ## Oracle parity (TA-Lib, then Wickra); narrow with ARGS="EMA ATR"
	uv run python scripts/verification/correctness.py $(ARGS)

verify-external: verify ## Compatibility alias for the unified oracle registry

# ------------------------------------------------------------- benchmarks ----

bench: ## Benchmark against TA-Lib; narrow with ARGS="SMA MAX"
	uv run python scripts/verification/benchmark.py $(ARGS)

# ------------------------------------------------------------- housekeeping ----

docs: ## Regenerate the indicator and metric API references
	uv run python scripts/gen_indicators_doc.py
	uv run python scripts/gen_metrics_doc.py

fmt: ## Format Rust sources
	cargo fmt --all

lint: ## Clippy across the workspace, warnings denied
	cargo clippy --workspace --all-targets -- -D warnings

clean: ## Remove build artifacts
	cargo clean
	rm -rf dist build *.egg-info
	find python -name '*.so' -delete
