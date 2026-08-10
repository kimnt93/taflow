# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.89M | 0.038 | 26.66M | 0.054 | 1.57× | 1.45× |
| 10,000 | 0.324 | 30.89M | 0.320 | 31.26M | 0.171 | 0.53× | 0.54× |
| 100,000 | 3.357 | 29.79M | 3.000 | 33.33M | 1.257 | 0.37× | 0.42× |
| 1,000,000 | 34.408 | 29.06M | 31.780 | 31.47M | 12.566 | 0.37× | 0.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.169 | 2.08× |
| 1 | 5 | 0.326 | 0.564 | 1.73× |
| 1 | 10 | 0.528 | 1.055 | 2.00× |
| 10 | 1 | 0.061 | 0.103 | 1.68× |
| 10 | 5 | 0.277 | 0.539 | 1.94× |
| 10 | 10 | 0.545 | 1.011 | 1.85× |
| 100 | 1 | 0.063 | 0.113 | 1.81× |
| 100 | 5 | 0.311 | 0.514 | 1.65× |
| 100 | 10 | 0.557 | 1.046 | 1.88× |
| 1,000 | 1 | 0.085 | 0.116 | 1.36× |
| 1,000 | 5 | 0.263 | 0.600 | 2.28× |
| 1,000 | 10 | 0.904 | 1.170 | 1.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
