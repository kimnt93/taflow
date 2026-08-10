# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.32M | 0.017 | 57.26M | 0.040 | 1.93× | 2.28× |
| 10,000 | 0.167 | 59.96M | 0.175 | 57.16M | 0.151 | 0.90× | 0.86× |
| 100,000 | 2.051 | 48.75M | 1.652 | 60.53M | 0.966 | 0.47× | 0.58× |
| 1,000,000 | 18.617 | 53.71M | 16.778 | 59.60M | 9.178 | 0.49× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.139 | 1.57× |
| 1 | 5 | 0.332 | 0.466 | 1.40× |
| 1 | 10 | 0.544 | 1.017 | 1.87× |
| 10 | 1 | 0.067 | 0.119 | 1.78× |
| 10 | 5 | 0.277 | 0.473 | 1.71× |
| 10 | 10 | 0.547 | 1.041 | 1.90× |
| 100 | 1 | 0.064 | 0.111 | 1.74× |
| 100 | 5 | 0.295 | 0.512 | 1.73× |
| 100 | 10 | 0.558 | 0.998 | 1.79× |
| 1,000 | 1 | 0.073 | 0.101 | 1.39× |
| 1,000 | 5 | 0.284 | 0.557 | 1.96× |
| 1,000 | 10 | 0.597 | 1.094 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
