# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 245.58M | 0.003 | 308.22M | 0.026 | 6.40× | 8.03× |
| 10,000 | 0.028 | 356.49M | 0.025 | 397.69M | 0.030 | 1.06× | 1.18× |
| 100,000 | 0.257 | 389.24M | 0.239 | 419.10M | 0.069 | 0.27× | 0.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.051 | 0.097 | 1.90× |
| 1 | 5 | 0.260 | 0.464 | 1.78× |
| 1 | 10 | 0.384 | 0.862 | 2.25× |
| 10 | 1 | 0.041 | 0.083 | 2.00× |
| 10 | 5 | 0.183 | 0.423 | 2.31× |
| 10 | 10 | 0.463 | 1.257 | 2.72× |
| 100 | 1 | 0.064 | 0.089 | 1.39× |
| 100 | 5 | 0.219 | 0.455 | 2.08× |
| 100 | 10 | 0.427 | 0.986 | 2.31× |
| 1,000 | 1 | 0.052 | 0.092 | 1.78× |
| 1,000 | 5 | 0.203 | 0.429 | 2.12× |
| 1,000 | 10 | 0.427 | 0.986 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
