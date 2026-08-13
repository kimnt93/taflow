# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.165 | 6.06M | 0.155 | 6.46M | 0.207 | 1.25× | 1.34× |
| 10,000 | 1.487 | 6.72M | 1.894 | 5.28M | 1.179 | 0.79× | 0.62× |
| 100,000 | 15.207 | 6.58M | 15.350 | 6.51M | 5.301 | 0.35× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.266 | 1.67× |
| 1 | 5 | 0.362 | 1.265 | 3.50× |
| 1 | 10 | 0.647 | 2.474 | 3.82× |
| 10 | 1 | 0.073 | 0.235 | 3.23× |
| 10 | 5 | 0.320 | 1.359 | 4.24× |
| 10 | 10 | 0.624 | 2.498 | 4.00× |
| 100 | 1 | 0.083 | 0.249 | 3.01× |
| 100 | 5 | 0.317 | 1.367 | 4.32× |
| 100 | 10 | 0.660 | 2.477 | 3.76× |
| 1,000 | 1 | 0.225 | 0.296 | 1.32× |
| 1,000 | 5 | 0.433 | 1.669 | 3.86× |
| 1,000 | 10 | 0.764 | 3.030 | 3.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
