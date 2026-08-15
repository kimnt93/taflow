# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.70M | 0.007 | 147.61M | 0.040 | 4.77× | 5.88× |
| 10,000 | 0.061 | 165.27M | 0.059 | 169.13M | 0.092 | 1.52× | 1.55× |
| 100,000 | 0.601 | 166.40M | 0.559 | 178.99M | 0.569 | 0.95× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.126 | 1.55× |
| 1 | 5 | 0.234 | 0.541 | 2.32× |
| 1 | 10 | 0.387 | 0.930 | 2.41× |
| 10 | 1 | 0.040 | 0.087 | 2.19× |
| 10 | 5 | 0.185 | 0.433 | 2.34× |
| 10 | 10 | 0.400 | 0.992 | 2.48× |
| 100 | 1 | 0.046 | 0.088 | 1.91× |
| 100 | 5 | 0.211 | 0.458 | 2.17× |
| 100 | 10 | 0.391 | 0.987 | 2.52× |
| 1,000 | 1 | 0.051 | 0.111 | 2.16× |
| 1,000 | 5 | 0.201 | 0.467 | 2.33× |
| 1,000 | 10 | 0.438 | 0.979 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
