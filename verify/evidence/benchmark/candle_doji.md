# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.06M | 0.004 | 233.37M | 0.032 | 4.17× | 7.49× |
| 10,000 | 0.037 | 272.93M | 0.032 | 314.32M | 0.054 | 1.48× | 1.71× |
| 100,000 | 0.338 | 296.04M | 0.317 | 315.39M | 0.245 | 0.72× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.106 | 0.92× |
| 1 | 5 | 0.282 | 0.439 | 1.56× |
| 1 | 10 | 0.393 | 1.008 | 2.56× |
| 10 | 1 | 0.046 | 0.089 | 1.95× |
| 10 | 5 | 0.197 | 0.440 | 2.23× |
| 10 | 10 | 0.399 | 0.929 | 2.33× |
| 100 | 1 | 0.055 | 0.097 | 1.77× |
| 100 | 5 | 0.215 | 0.448 | 2.08× |
| 100 | 10 | 0.401 | 0.896 | 2.23× |
| 1,000 | 1 | 0.046 | 0.087 | 1.88× |
| 1,000 | 5 | 0.189 | 0.433 | 2.28× |
| 1,000 | 10 | 0.453 | 0.937 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
