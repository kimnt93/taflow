# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.34M | 0.007 | 150.26M | 0.038 | 4.15× | 5.71× |
| 10,000 | 0.063 | 157.99M | 0.121 | 82.68M | 0.108 | 1.70× | 0.89× |
| 100,000 | 0.862 | 115.98M | 0.847 | 118.04M | 0.863 | 1.00× | 1.02× |
| 1,000,000 | 8.978 | 111.38M | 8.693 | 115.03M | 8.302 | 0.92× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.108 | 0.86× |
| 1 | 5 | 0.329 | 0.503 | 1.53× |
| 1 | 10 | 0.520 | 0.928 | 1.78× |
| 10 | 1 | 0.054 | 0.091 | 1.69× |
| 10 | 5 | 0.233 | 0.443 | 1.90× |
| 10 | 10 | 0.527 | 0.929 | 1.76× |
| 100 | 1 | 0.056 | 0.092 | 1.65× |
| 100 | 5 | 0.244 | 0.425 | 1.74× |
| 100 | 10 | 0.515 | 0.908 | 1.76× |
| 1,000 | 1 | 0.064 | 0.105 | 1.65× |
| 1,000 | 5 | 0.269 | 0.517 | 1.92× |
| 1,000 | 10 | 0.554 | 1.017 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
