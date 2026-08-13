# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 18.438 | 54.24K | 17.853 | 56.01K | 0.823 | 0.04× | 0.05× |
| 10,000 | 183.165 | 54.60K | 192.848 | 51.85K | 6.128 | 0.03× | 0.03× |
| 100,000 | 1841.003 | 54.32K | 1825.849 | 54.77K | 60.249 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.285 | 1.84× |
| 1 | 5 | 0.343 | 1.223 | 3.56× |
| 1 | 10 | 0.615 | 2.256 | 3.67× |
| 10 | 1 | 0.075 | 0.213 | 2.84× |
| 10 | 5 | 0.302 | 1.260 | 4.17× |
| 10 | 10 | 0.640 | 2.287 | 3.58× |
| 100 | 1 | 1.643 | 0.272 | 0.17× |
| 100 | 5 | 2.264 | 1.514 | 0.67× |
| 100 | 10 | 3.395 | 2.844 | 0.84× |
| 1,000 | 1 | 18.060 | 0.919 | 0.05× |
| 1,000 | 5 | 19.422 | 4.691 | 0.24× |
| 1,000 | 10 | 37.899 | 9.123 | 0.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
