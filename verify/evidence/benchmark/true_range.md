# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.79M | 0.007 | 136.07M | 0.029 | 2.76× | 4.00× |
| 10,000 | 0.044 | 229.25M | 0.037 | 270.65M | 0.037 | 0.84× | 0.99× |
| 100,000 | 0.351 | 284.78M | 0.329 | 304.05M | 0.093 | 0.27× | 0.28× |
| 1,000,000 | 4.171 | 239.77M | 3.579 | 279.38M | 1.832 | 0.44× | 0.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.114 | 1.80× |
| 1 | 5 | 0.375 | 0.461 | 1.23× |
| 1 | 10 | 0.497 | 0.922 | 1.86× |
| 10 | 1 | 0.052 | 0.099 | 1.89× |
| 10 | 5 | 0.273 | 0.432 | 1.58× |
| 10 | 10 | 0.477 | 0.940 | 1.97× |
| 100 | 1 | 0.054 | 0.085 | 1.56× |
| 100 | 5 | 0.235 | 0.443 | 1.88× |
| 100 | 10 | 0.532 | 0.907 | 1.71× |
| 1,000 | 1 | 0.051 | 0.090 | 1.77× |
| 1,000 | 5 | 0.242 | 0.469 | 1.94× |
| 1,000 | 10 | 0.568 | 0.934 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
