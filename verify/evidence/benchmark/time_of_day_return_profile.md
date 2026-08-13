# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.233 | 4.30M | 0.231 | 4.32M | 1.560 | 6.71× | 6.74× |
| 10,000 | 2.076 | 4.82M | 1.820 | 5.49M | 15.541 | 7.49× | 8.54× |
| 100,000 | 19.711 | 5.07M | 18.266 | 5.47M | 188.031 | 9.54× | 10.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.294 | 2.16× |
| 1 | 5 | 0.518 | 1.435 | 2.77× |
| 1 | 10 | 0.722 | 2.637 | 3.65× |
| 10 | 1 | 0.088 | 0.276 | 3.15× |
| 10 | 5 | 0.353 | 1.510 | 4.28× |
| 10 | 10 | 0.715 | 2.841 | 3.97× |
| 100 | 1 | 0.098 | 0.392 | 3.99× |
| 100 | 5 | 0.341 | 2.140 | 6.27× |
| 100 | 10 | 0.702 | 4.059 | 5.78× |
| 1,000 | 1 | 0.273 | 2.001 | 7.34× |
| 1,000 | 5 | 0.460 | 9.381 | 20.39× |
| 1,000 | 10 | 0.933 | 18.749 | 20.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
