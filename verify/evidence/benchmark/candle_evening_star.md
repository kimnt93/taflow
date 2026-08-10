# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.79M | 0.019 | 52.62M | 0.040 | 1.76× | 2.12× |
| 10,000 | 0.176 | 56.97M | 0.167 | 59.78M | 0.131 | 0.75× | 0.78× |
| 100,000 | 1.707 | 58.59M | 1.616 | 61.89M | 0.938 | 0.55× | 0.58× |
| 1,000,000 | 15.832 | 63.16M | 15.663 | 63.85M | 9.496 | 0.60× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.146 | 1.37× |
| 1 | 5 | 0.339 | 0.606 | 1.79× |
| 1 | 10 | 0.552 | 1.040 | 1.88× |
| 10 | 1 | 0.060 | 0.095 | 1.58× |
| 10 | 5 | 0.283 | 0.485 | 1.71× |
| 10 | 10 | 0.575 | 0.993 | 1.72× |
| 100 | 1 | 0.064 | 0.093 | 1.46× |
| 100 | 5 | 0.461 | 0.535 | 1.16× |
| 100 | 10 | 0.594 | 1.015 | 1.71× |
| 1,000 | 1 | 0.066 | 0.107 | 1.61× |
| 1,000 | 5 | 0.327 | 0.582 | 1.78× |
| 1,000 | 10 | 0.645 | 1.085 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
