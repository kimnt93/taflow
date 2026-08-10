# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.83M | 0.017 | 60.34M | 0.044 | 2.24× | 2.66× |
| 10,000 | 0.119 | 84.14M | 0.117 | 85.48M | 0.131 | 1.10× | 1.12× |
| 100,000 | 1.132 | 88.30M | 1.090 | 91.72M | 1.072 | 0.95× | 0.98× |
| 1,000,000 | 11.687 | 85.57M | 11.255 | 88.85M | 10.509 | 0.90× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.134 | 1.38× |
| 1 | 5 | 0.288 | 0.498 | 1.73× |
| 1 | 10 | 0.473 | 1.046 | 2.21× |
| 10 | 1 | 0.061 | 0.096 | 1.57× |
| 10 | 5 | 0.281 | 0.495 | 1.77× |
| 10 | 10 | 0.513 | 0.942 | 1.84× |
| 100 | 1 | 0.051 | 0.091 | 1.78× |
| 100 | 5 | 0.315 | 0.615 | 1.96× |
| 100 | 10 | 0.608 | 0.972 | 1.60× |
| 1,000 | 1 | 0.062 | 0.108 | 1.73× |
| 1,000 | 5 | 0.257 | 0.562 | 2.19× |
| 1,000 | 10 | 0.645 | 1.070 | 1.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
