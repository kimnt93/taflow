# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 471.08M | 0.001 | 841.75M | 0.030 | 13.93× | 24.89× |
| 10,000 | 0.006 | 1.63G | 0.004 | 2.60G | 0.032 | 5.27× | 8.39× |
| 100,000 | 0.050 | 2.00G | 0.031 | 3.24G | 0.058 | 1.17× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.115 | 1.53× |
| 1 | 5 | 0.312 | 0.464 | 1.49× |
| 1 | 10 | 0.388 | 0.924 | 2.38× |
| 10 | 1 | 0.043 | 0.097 | 2.27× |
| 10 | 5 | 0.181 | 0.423 | 2.34× |
| 10 | 10 | 0.381 | 0.911 | 2.39× |
| 100 | 1 | 0.042 | 0.086 | 2.03× |
| 100 | 5 | 0.181 | 0.430 | 2.37× |
| 100 | 10 | 0.376 | 1.055 | 2.80× |
| 1,000 | 1 | 0.059 | 0.102 | 1.75× |
| 1,000 | 5 | 0.192 | 0.431 | 2.24× |
| 1,000 | 10 | 0.391 | 0.913 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
