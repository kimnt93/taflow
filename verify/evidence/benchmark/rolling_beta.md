# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.77M | 0.008 | 125.79M | 0.037 | 3.80× | 4.60× |
| 10,000 | 0.053 | 189.67M | 0.055 | 182.81M | 0.091 | 1.72× | 1.66× |
| 100,000 | 0.523 | 191.32M | 0.476 | 210.20M | 0.547 | 1.05× | 1.15× |
| 1,000,000 | 5.262 | 190.04M | 4.926 | 203.00M | 5.949 | 1.13× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.140 | 1.24× |
| 1 | 5 | 0.288 | 0.568 | 1.97× |
| 1 | 10 | 0.510 | 0.941 | 1.85× |
| 10 | 1 | 0.049 | 0.095 | 1.96× |
| 10 | 5 | 0.221 | 0.430 | 1.95× |
| 10 | 10 | 0.469 | 0.921 | 1.96× |
| 100 | 1 | 0.052 | 0.095 | 1.83× |
| 100 | 5 | 0.227 | 0.445 | 1.96× |
| 100 | 10 | 0.495 | 0.926 | 1.87× |
| 1,000 | 1 | 0.056 | 0.098 | 1.73× |
| 1,000 | 5 | 0.239 | 0.456 | 1.91× |
| 1,000 | 10 | 0.481 | 0.989 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
