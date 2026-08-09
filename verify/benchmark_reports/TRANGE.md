# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.43M | 0.005 | 206.30M | 0.029 | 4.32× | 6.01× |
| 10,000 | 0.019 | 525.89M | 0.016 | 630.37M | 0.034 | 1.80× | 2.16× |
| 100,000 | 0.142 | 706.39M | 0.116 | 860.75M | 0.090 | 0.64× | 0.78× |
| 1,000,000 | 2.317 | 431.61M | 1.651 | 605.84M | 1.553 | 0.67× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.205 | 1.86× |
| 1 | 5 | 0.265 | 0.503 | 1.90× |
| 1 | 10 | 0.497 | 0.947 | 1.91× |
| 10 | 1 | 0.052 | 0.090 | 1.71× |
| 10 | 5 | 0.262 | 0.462 | 1.76× |
| 10 | 10 | 0.525 | 0.929 | 1.77× |
| 100 | 1 | 0.054 | 0.089 | 1.64× |
| 100 | 5 | 0.229 | 0.425 | 1.85× |
| 100 | 10 | 0.532 | 1.009 | 1.90× |
| 1,000 | 1 | 0.055 | 0.094 | 1.72× |
| 1,000 | 5 | 0.238 | 0.431 | 1.81× |
| 1,000 | 10 | 0.528 | 0.945 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
