# RollingInterquartileRange benchmark (`RollingIqr` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.344 | 2.91M | 0.334 | 2.99M | 0.307 | 0.89× | 0.92× |
| 10,000 | 3.449 | 2.90M | 3.634 | 2.75M | 1.786 | 0.52× | 0.49× |
| 100,000 | 33.529 | 2.98M | 33.452 | 2.99M | 15.750 | 0.47× | 0.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.281 | 2.09× |
| 1 | 5 | 0.510 | 1.086 | 2.13× |
| 1 | 10 | 0.625 | 2.604 | 4.16× |
| 10 | 1 | 0.079 | 0.223 | 2.82× |
| 10 | 5 | 0.304 | 1.075 | 3.53× |
| 10 | 10 | 0.634 | 2.326 | 3.67× |
| 100 | 1 | 0.110 | 0.237 | 2.15× |
| 100 | 5 | 0.302 | 1.431 | 4.75× |
| 100 | 10 | 0.637 | 2.542 | 3.99× |
| 1,000 | 1 | 0.423 | 0.384 | 0.91× |
| 1,000 | 5 | 0.672 | 2.259 | 3.36× |
| 1,000 | 10 | 1.061 | 4.126 | 3.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
