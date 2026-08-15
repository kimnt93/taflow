# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.13M | 0.006 | 157.55M | 0.037 | 4.72× | 5.85× |
| 10,000 | 0.058 | 173.54M | 0.053 | 187.57M | 0.115 | 1.99× | 2.15× |
| 100,000 | 0.531 | 188.35M | 0.526 | 190.16M | 0.900 | 1.70× | 1.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.106 | 1.41× |
| 1 | 5 | 0.213 | 0.501 | 2.35× |
| 1 | 10 | 0.441 | 0.990 | 2.24× |
| 10 | 1 | 0.044 | 0.090 | 2.04× |
| 10 | 5 | 0.178 | 0.441 | 2.48× |
| 10 | 10 | 0.414 | 1.085 | 2.62× |
| 100 | 1 | 0.045 | 0.096 | 2.14× |
| 100 | 5 | 0.192 | 0.457 | 2.38× |
| 100 | 10 | 0.435 | 0.934 | 2.15× |
| 1,000 | 1 | 0.061 | 0.130 | 2.14× |
| 1,000 | 5 | 0.217 | 0.531 | 2.45× |
| 1,000 | 10 | 0.463 | 1.080 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
