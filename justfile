# TAFlow development recipes. Requires `just` (https://github.com/casey/just).

# Rebuild the extension into the verify project's environment.
build:
    cd verify && uv sync --reinstall-package taflow

# Rebuild with the local CPU's full feature set (FMA/AVX2 on modern x86).
# For local measurement only — distributed wheels rely on multiversion
# runtime dispatch instead; never bake target-cpu into wheel builds.
build-native:
    cd verify && RUSTFLAGS="-C target-cpu=native" uv sync --reinstall-package taflow

# Correctness for a subset (e.g. `just verify EMA ATR`) or everything.
verify *ARGS:
    cd verify && uv run python verify.py {{ARGS}}

# Extension correctness against pandas-ta-classic, Polars, and SMC.
verify-external *ARGS:
    cd verify && uv sync --extra extra-oracles && uv run python external_oracles.py {{ARGS}}
    cd verify && uv run python source_comparison.py

# Benchmarks (writes verify/benchmark_reports/).
bench *ARGS:
    cd verify && uv run python benchmark.py {{ARGS}}

# Rust unit tests.
test:
    cargo test -p taflow

check:
    cargo check --workspace
