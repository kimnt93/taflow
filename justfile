# TAFlow development recipes. Requires `just` (https://github.com/casey/just).

# Rebuild the extension in the root development environment.
build:
    uv run maturin develop --release

# Rebuild with the local CPU's full feature set (FMA/AVX2 on modern x86).
# For local measurement only — distributed wheels rely on multiversion
# runtime dispatch instead; never bake target-cpu into wheel builds.
build-native:
    RUSTFLAGS="-C target-cpu=native" uv run maturin develop --release

# Correctness for a subset (e.g. `just verify EMA ATR`) or everything.
verify *ARGS:
    uv run python scripts/verification/correctness.py {{ARGS}}

# Compatibility alias for the unified TA-Lib/Wickra registry.
verify-external *ARGS:
    uv run python scripts/verification/correctness.py {{ARGS}}

# Benchmarks (writes verify/BENCHMARK.md and per-indicator evidence).
bench *ARGS:
    uv run python scripts/verification/benchmark.py {{ARGS}}

# Rust unit tests.
test:
    cargo test -p taflow

check:
    cargo check --workspace
