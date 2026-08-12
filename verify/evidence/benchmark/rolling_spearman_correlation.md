# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.675 | 1.48M | 0.423 | 2.36M | 0.890 | 1.32× | 2.10× |
| 10,000 | 4.477 | 2.23M | 4.413 | 2.27M | 6.847 | 1.53× | 1.55× |
| 100,000 | 43.051 | 2.32M | 45.736 | 2.19M | 66.346 | 1.54× | 1.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.273 | 1.97× |
| 1 | 5 | 0.295 | 1.309 | 4.44× |
| 1 | 10 | 0.490 | 2.399 | 4.89× |
| 10 | 1 | 0.056 | 0.219 | 3.89× |
| 10 | 5 | 0.245 | 1.343 | 5.48× |
| 10 | 10 | 0.562 | 2.473 | 4.40× |
| 100 | 1 | 0.091 | 0.287 | 3.16× |
| 100 | 5 | 0.274 | 1.659 | 6.06× |
| 100 | 10 | 0.578 | 3.030 | 5.24× |
| 1,000 | 1 | 0.478 | 0.864 | 1.81× |
| 1,000 | 5 | 1.004 | 4.599 | 4.58× |
| 1,000 | 10 | 1.230 | 9.093 | 7.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
