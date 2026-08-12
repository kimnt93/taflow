# PositionHold benchmark (`nonzero position hold` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 190.35M | 0.006 | 177.70M | 0.129 | 24.62× | 22.99× |
| 10,000 | 0.026 | 382.36M | 0.023 | 426.59M | 1.226 | 46.88× | 52.30× |
| 100,000 | 0.218 | 457.88M | 0.198 | 506.20M | 12.296 | 56.30× | 62.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.080 | 1.23× |
| 1 | 5 | 0.327 | 0.290 | 0.88× |
| 1 | 10 | 0.446 | 0.600 | 1.35× |
| 10 | 1 | 0.048 | 0.063 | 1.31× |
| 10 | 5 | 0.217 | 0.282 | 1.30× |
| 10 | 10 | 0.455 | 0.578 | 1.27× |
| 100 | 1 | 0.049 | 0.072 | 1.48× |
| 100 | 5 | 0.209 | 0.345 | 1.65× |
| 100 | 10 | 0.466 | 0.722 | 1.55× |
| 1,000 | 1 | 0.048 | 0.182 | 3.80× |
| 1,000 | 5 | 0.254 | 0.994 | 3.92× |
| 1,000 | 10 | 0.486 | 1.807 | 3.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
