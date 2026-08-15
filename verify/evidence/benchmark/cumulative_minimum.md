# CumulativeMinimum benchmark (`numpy.minimum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 233.09M | 0.003 | 299.48M | 0.015 | 3.59× | 4.61× |
| 10,000 | 0.030 | 338.37M | 0.028 | 361.84M | 0.040 | 1.34× | 1.43× |
| 100,000 | 0.280 | 356.69M | 0.267 | 374.11M | 0.302 | 1.08× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.083 | 0.80× |
| 1 | 5 | 0.232 | 0.288 | 1.24× |
| 1 | 10 | 0.406 | 0.580 | 1.43× |
| 10 | 1 | 0.040 | 0.059 | 1.48× |
| 10 | 5 | 0.191 | 0.282 | 1.47× |
| 10 | 10 | 0.391 | 0.597 | 1.52× |
| 100 | 1 | 0.055 | 0.069 | 1.25× |
| 100 | 5 | 0.178 | 0.267 | 1.50× |
| 100 | 10 | 0.369 | 0.598 | 1.62× |
| 1,000 | 1 | 0.041 | 0.063 | 1.54× |
| 1,000 | 5 | 0.187 | 0.298 | 1.60× |
| 1,000 | 10 | 0.451 | 0.744 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
