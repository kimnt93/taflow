# RollingMedian benchmark (`MedianMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.17M | 0.034 | 29.03M | 0.341 | 9.95× | 9.91× |
| 10,000 | 0.401 | 24.93M | 0.395 | 25.31M | 1.869 | 4.66× | 4.73× |
| 100,000 | 3.974 | 25.17M | 4.005 | 24.97M | 18.673 | 4.70× | 4.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.263 | 4.64× |
| 1 | 5 | 0.295 | 1.162 | 3.94× |
| 1 | 10 | 0.435 | 2.590 | 5.95× |
| 10 | 1 | 0.047 | 0.217 | 4.65× |
| 10 | 5 | 0.199 | 1.178 | 5.91× |
| 10 | 10 | 0.397 | 2.211 | 5.57× |
| 100 | 1 | 0.055 | 0.254 | 4.62× |
| 100 | 5 | 0.225 | 1.555 | 6.90× |
| 100 | 10 | 0.445 | 2.484 | 5.58× |
| 1,000 | 1 | 0.108 | 0.393 | 3.63× |
| 1,000 | 5 | 0.224 | 2.146 | 9.59× |
| 1,000 | 10 | 0.542 | 4.415 | 8.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
