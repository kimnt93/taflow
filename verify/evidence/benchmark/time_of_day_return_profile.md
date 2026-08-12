# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.73M | 0.042 | 23.82M | 1.685 | 33.26× | 40.14× |
| 10,000 | 0.360 | 27.75M | 0.309 | 32.41M | 16.326 | 45.31× | 52.90× |
| 100,000 | 3.961 | 25.25M | 2.985 | 33.51M | 195.948 | 49.48× | 65.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.302 | 3.17× |
| 1 | 5 | 0.447 | 1.644 | 3.68× |
| 1 | 10 | 0.552 | 2.847 | 5.15× |
| 10 | 1 | 0.068 | 0.293 | 4.29× |
| 10 | 5 | 0.280 | 1.635 | 5.85× |
| 10 | 10 | 0.582 | 3.179 | 5.47× |
| 100 | 1 | 0.070 | 0.429 | 6.17× |
| 100 | 5 | 0.283 | 2.287 | 8.09× |
| 100 | 10 | 0.602 | 4.357 | 7.23× |
| 1,000 | 1 | 0.096 | 2.084 | 21.62× |
| 1,000 | 5 | 0.307 | 9.587 | 31.23× |
| 1,000 | 10 | 0.738 | 19.104 | 25.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
