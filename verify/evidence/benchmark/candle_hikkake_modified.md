# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.84M | 0.011 | 93.10M | 0.033 | 2.36× | 3.06× |
| 10,000 | 0.062 | 161.21M | 0.058 | 171.66M | 0.083 | 1.33× | 1.42× |
| 100,000 | 0.561 | 178.36M | 0.546 | 183.29M | 0.571 | 1.02× | 1.05× |
| 1,000,000 | 5.780 | 173.01M | 5.768 | 173.37M | 5.526 | 0.96× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.120 | 1.05× |
| 1 | 5 | 0.419 | 0.448 | 1.07× |
| 1 | 10 | 0.534 | 0.912 | 1.71× |
| 10 | 1 | 0.054 | 0.092 | 1.69× |
| 10 | 5 | 0.251 | 0.422 | 1.68× |
| 10 | 10 | 0.523 | 0.910 | 1.74× |
| 100 | 1 | 0.052 | 0.087 | 1.66× |
| 100 | 5 | 0.250 | 0.427 | 1.71× |
| 100 | 10 | 0.498 | 0.932 | 1.87× |
| 1,000 | 1 | 0.060 | 0.100 | 1.66× |
| 1,000 | 5 | 0.261 | 0.475 | 1.82× |
| 1,000 | 10 | 0.575 | 0.975 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
