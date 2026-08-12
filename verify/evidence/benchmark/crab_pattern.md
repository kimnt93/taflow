# CrabPattern benchmark (`Crab` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.26M | 0.013 | 78.59M | 0.257 | 17.05× | 20.23× |
| 10,000 | 0.103 | 97.37M | 0.100 | 100.29M | 1.442 | 14.04× | 14.46× |
| 100,000 | 0.957 | 104.53M | 0.915 | 109.23M | 13.441 | 14.05× | 14.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.216 | 2.71× |
| 1 | 5 | 0.347 | 0.872 | 2.51× |
| 1 | 10 | 0.545 | 1.705 | 3.13× |
| 10 | 1 | 0.058 | 0.166 | 2.85× |
| 10 | 5 | 0.261 | 1.101 | 4.21× |
| 10 | 10 | 0.559 | 1.717 | 3.07× |
| 100 | 1 | 0.054 | 0.182 | 3.37× |
| 100 | 5 | 0.269 | 1.235 | 4.60× |
| 100 | 10 | 0.535 | 1.815 | 3.39× |
| 1,000 | 1 | 0.067 | 0.304 | 4.51× |
| 1,000 | 5 | 0.271 | 1.812 | 6.67× |
| 1,000 | 10 | 0.551 | 3.115 | 5.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
