# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 378.23M | 0.001 | 840.70M | 0.016 | 6.13× | 13.63× |
| 10,000 | 0.009 | 1.14G | 0.005 | 1.90G | 0.027 | 3.09× | 5.18× |
| 100,000 | 0.070 | 1.43G | 0.046 | 2.17G | 0.151 | 2.16× | 3.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.099 | 1.40× |
| 1 | 5 | 0.191 | 0.305 | 1.60× |
| 1 | 10 | 0.388 | 0.664 | 1.71× |
| 10 | 1 | 0.040 | 0.064 | 1.59× |
| 10 | 5 | 0.186 | 0.307 | 1.65× |
| 10 | 10 | 0.385 | 0.660 | 1.71× |
| 100 | 1 | 0.044 | 0.062 | 1.40× |
| 100 | 5 | 0.170 | 0.321 | 1.89× |
| 100 | 10 | 0.383 | 0.666 | 1.74× |
| 1,000 | 1 | 0.038 | 0.076 | 1.97× |
| 1,000 | 5 | 0.176 | 0.351 | 2.00× |
| 1,000 | 10 | 0.390 | 0.844 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
