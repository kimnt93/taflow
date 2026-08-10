# CumulativeSumControlChart benchmark (`CUSUM event filter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.05M | 0.006 | 157.22M | 0.523 | 75.33× | 82.22× |
| 10,000 | 0.041 | 241.86M | 0.039 | 256.26M | 5.041 | 121.92× | 129.18× |
| 100,000 | 0.397 | 251.75M | 0.371 | 269.32M | 50.245 | 126.49× | 135.32× |
| 1,000,000 | 4.249 | 235.37M | 3.743 | 267.15M | 507.292 | 119.40× | 135.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.116 | 1.41× |
| 1 | 5 | 0.265 | 0.464 | 1.75× |
| 1 | 10 | 0.433 | 0.833 | 1.92× |
| 10 | 1 | 0.046 | 0.090 | 1.94× |
| 10 | 5 | 0.217 | 0.433 | 1.99× |
| 10 | 10 | 0.476 | 0.926 | 1.95× |
| 100 | 1 | 0.057 | 0.131 | 2.31× |
| 100 | 5 | 0.215 | 0.670 | 3.11× |
| 100 | 10 | 0.485 | 1.440 | 2.97× |
| 1,000 | 1 | 0.056 | 0.613 | 10.95× |
| 1,000 | 5 | 0.241 | 2.984 | 12.40× |
| 1,000 | 10 | 0.516 | 5.993 | 11.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
