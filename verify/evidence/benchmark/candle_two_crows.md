# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.87M | 0.003 | 323.39M | 0.033 | 4.97× | 10.59× |
| 10,000 | 0.063 | 158.56M | 0.056 | 179.81M | 0.110 | 1.75× | 1.98× |
| 100,000 | 1.029 | 97.21M | 0.952 | 105.03M | 0.912 | 0.89× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.156 | 0.108 | 0.69× |
| 1 | 5 | 0.272 | 0.456 | 1.68× |
| 1 | 10 | 0.405 | 0.931 | 2.30× |
| 10 | 1 | 0.047 | 0.089 | 1.90× |
| 10 | 5 | 0.192 | 0.431 | 2.25× |
| 10 | 10 | 0.386 | 0.929 | 2.41× |
| 100 | 1 | 0.048 | 0.102 | 2.14× |
| 100 | 5 | 0.213 | 0.446 | 2.09× |
| 100 | 10 | 0.404 | 0.904 | 2.24× |
| 1,000 | 1 | 0.052 | 0.099 | 1.92× |
| 1,000 | 5 | 0.205 | 0.538 | 2.63× |
| 1,000 | 10 | 0.463 | 1.011 | 2.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
