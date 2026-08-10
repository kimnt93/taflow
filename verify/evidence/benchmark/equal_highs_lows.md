# EqualHighsLows benchmark (`causal equal pivot levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.45M | 0.044 | 22.56M | 4.652 | 99.77× | 104.93× |
| 10,000 | 0.440 | 22.70M | 0.431 | 23.21M | 45.308 | 102.86× | 105.17× |
| 100,000 | 4.381 | 22.82M | 4.198 | 23.82M | 472.595 | 107.87× | 112.57× |
| 1,000,000 | 46.449 | 21.53M | 44.369 | 22.54M | 4622.946 | 99.53× | 104.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.183 | 2.43× |
| 1 | 5 | 0.318 | 0.808 | 2.54× |
| 1 | 10 | 0.525 | 1.523 | 2.90× |
| 10 | 1 | 0.051 | 0.181 | 3.56× |
| 10 | 5 | 0.249 | 0.886 | 3.55× |
| 10 | 10 | 0.516 | 1.835 | 3.56× |
| 100 | 1 | 0.056 | 0.545 | 9.73× |
| 100 | 5 | 0.274 | 2.786 | 10.16× |
| 100 | 10 | 0.578 | 5.574 | 9.64× |
| 1,000 | 1 | 0.106 | 4.972 | 46.73× |
| 1,000 | 5 | 0.287 | 24.027 | 83.63× |
| 1,000 | 10 | 0.626 | 50.623 | 80.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
