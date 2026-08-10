# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.15M | 0.027 | 36.75M | 0.041 | 1.28× | 1.51× |
| 10,000 | 0.327 | 30.62M | 0.303 | 33.06M | 0.145 | 0.44× | 0.48× |
| 100,000 | 3.453 | 28.96M | 2.864 | 34.92M | 0.904 | 0.26× | 0.32× |
| 1,000,000 | 31.797 | 31.45M | 32.408 | 30.86M | 9.630 | 0.30× | 0.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.130 | 1.68× |
| 1 | 5 | 0.321 | 0.676 | 2.11× |
| 1 | 10 | 0.654 | 1.074 | 1.64× |
| 10 | 1 | 0.052 | 0.122 | 2.33× |
| 10 | 5 | 0.278 | 0.494 | 1.78× |
| 10 | 10 | 0.577 | 1.092 | 1.89× |
| 100 | 1 | 0.058 | 0.101 | 1.72× |
| 100 | 5 | 0.286 | 0.523 | 1.83× |
| 100 | 10 | 0.642 | 1.335 | 2.08× |
| 1,000 | 1 | 0.103 | 0.127 | 1.24× |
| 1,000 | 5 | 0.368 | 0.670 | 1.82× |
| 1,000 | 10 | 0.724 | 1.345 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
