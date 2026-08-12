# ExponentiallyWeightedSum benchmark (`exponentially weighted sum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.66M | 0.006 | 178.59M | 0.181 | 28.43× | 32.41× |
| 10,000 | 0.038 | 266.21M | 0.035 | 283.96M | 1.635 | 43.54× | 46.44× |
| 100,000 | 0.412 | 242.54M | 0.321 | 311.50M | 17.937 | 43.50× | 55.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.102 | 1.17× |
| 1 | 5 | 0.315 | 0.435 | 1.38× |
| 1 | 10 | 0.465 | 0.846 | 1.82× |
| 10 | 1 | 0.050 | 0.084 | 1.67× |
| 10 | 5 | 0.220 | 0.406 | 1.85× |
| 10 | 10 | 0.469 | 0.874 | 1.86× |
| 100 | 1 | 0.050 | 0.101 | 2.02× |
| 100 | 5 | 0.241 | 0.519 | 2.15× |
| 100 | 10 | 0.494 | 1.032 | 2.09× |
| 1,000 | 1 | 0.050 | 0.264 | 5.24× |
| 1,000 | 5 | 0.261 | 1.450 | 5.54× |
| 1,000 | 10 | 0.534 | 2.974 | 5.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
