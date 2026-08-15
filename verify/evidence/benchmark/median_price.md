# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 433.21M | 0.001 | 1.00G | 0.029 | 12.44× | 28.80× |
| 10,000 | 0.007 | 1.39G | 0.004 | 2.45G | 0.033 | 4.55× | 8.00× |
| 100,000 | 0.063 | 1.59G | 0.038 | 2.65G | 0.070 | 1.12× | 1.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.168 | 1.47× |
| 1 | 5 | 0.295 | 0.453 | 1.54× |
| 1 | 10 | 0.405 | 0.884 | 2.18× |
| 10 | 1 | 0.041 | 0.092 | 2.23× |
| 10 | 5 | 0.184 | 0.456 | 2.48× |
| 10 | 10 | 0.380 | 0.889 | 2.34× |
| 100 | 1 | 0.040 | 0.087 | 2.19× |
| 100 | 5 | 0.178 | 0.419 | 2.35× |
| 100 | 10 | 0.378 | 0.948 | 2.51× |
| 1,000 | 1 | 0.042 | 0.085 | 2.04× |
| 1,000 | 5 | 0.177 | 0.408 | 2.31× |
| 1,000 | 10 | 0.377 | 0.852 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
