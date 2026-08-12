# ExponentiallyWeightedVariance benchmark (`ewm variance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.47M | 0.007 | 149.17M | 1.276 | 162.68× | 190.37× |
| 10,000 | 0.052 | 192.72M | 0.049 | 203.87M | 12.958 | 249.73× | 264.18× |
| 100,000 | 0.446 | 224.14M | 0.440 | 227.02M | 126.257 | 282.99× | 286.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.116 | 1.72× |
| 1 | 5 | 0.285 | 0.495 | 1.74× |
| 1 | 10 | 0.476 | 0.842 | 1.77× |
| 10 | 1 | 0.049 | 0.094 | 1.93× |
| 10 | 5 | 0.226 | 0.483 | 2.14× |
| 10 | 10 | 0.518 | 0.989 | 1.91× |
| 100 | 1 | 0.061 | 0.238 | 3.92× |
| 100 | 5 | 0.257 | 1.122 | 4.38× |
| 100 | 10 | 0.544 | 2.109 | 3.88× |
| 1,000 | 1 | 0.055 | 1.408 | 25.64× |
| 1,000 | 5 | 0.268 | 6.750 | 25.17× |
| 1,000 | 10 | 0.493 | 13.766 | 27.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
