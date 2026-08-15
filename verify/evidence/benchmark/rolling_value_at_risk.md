# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.140 | 7.13M | 0.130 | 7.68M | 0.362 | 2.58× | 2.78× |
| 10,000 | 1.414 | 7.07M | 1.308 | 7.65M | 1.790 | 1.27× | 1.37× |
| 100,000 | 14.001 | 7.14M | 12.919 | 7.74M | 17.554 | 1.25× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.374 | 6.67× |
| 1 | 5 | 0.228 | 1.235 | 5.41× |
| 1 | 10 | 0.437 | 3.106 | 7.10× |
| 10 | 1 | 0.049 | 0.238 | 4.90× |
| 10 | 5 | 0.219 | 1.202 | 5.50× |
| 10 | 10 | 0.481 | 2.827 | 5.87× |
| 100 | 1 | 0.064 | 0.259 | 4.08× |
| 100 | 5 | 0.206 | 1.704 | 8.26× |
| 100 | 10 | 0.531 | 2.984 | 5.62× |
| 1,000 | 1 | 0.231 | 0.483 | 2.09× |
| 1,000 | 5 | 0.369 | 2.473 | 6.70× |
| 1,000 | 10 | 0.626 | 4.824 | 7.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
