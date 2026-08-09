# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.07M | 0.005 | 189.36M | 0.031 | 3.95× | 5.80× |
| 10,000 | 0.015 | 646.24M | 0.012 | 864.10M | 0.036 | 2.30× | 3.07× |
| 100,000 | 0.086 | 1.16G | 0.068 | 1.48G | 0.101 | 1.17× | 1.49× |
| 1,000,000 | 2.611 | 383.01M | 2.282 | 438.12M | 1.886 | 0.72× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.169 | 1.37× |
| 1 | 5 | 0.347 | 0.598 | 1.72× |
| 1 | 10 | 0.572 | 1.065 | 1.86× |
| 10 | 1 | 0.056 | 0.098 | 1.75× |
| 10 | 5 | 0.301 | 0.500 | 1.66× |
| 10 | 10 | 0.614 | 1.079 | 1.76× |
| 100 | 1 | 0.057 | 0.107 | 1.87× |
| 100 | 5 | 0.285 | 0.493 | 1.73× |
| 100 | 10 | 0.581 | 1.123 | 1.93× |
| 1,000 | 1 | 0.087 | 0.099 | 1.14× |
| 1,000 | 5 | 0.293 | 0.481 | 1.64× |
| 1,000 | 10 | 0.649 | 1.198 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
