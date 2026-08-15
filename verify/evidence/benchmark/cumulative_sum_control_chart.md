# CumulativeSumControlChart benchmark (`CUSUM event filter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.60M | 0.005 | 215.84M | 0.494 | 90.70× | 106.63× |
| 10,000 | 0.042 | 238.26M | 0.038 | 266.62M | 4.806 | 114.51× | 128.14× |
| 100,000 | 0.403 | 248.01M | 0.440 | 227.29M | 50.010 | 124.03× | 113.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.116 | 0.70× |
| 1 | 5 | 0.228 | 0.531 | 2.33× |
| 1 | 10 | 0.403 | 0.843 | 2.09× |
| 10 | 1 | 0.044 | 0.092 | 2.08× |
| 10 | 5 | 0.189 | 0.440 | 2.33× |
| 10 | 10 | 0.411 | 0.896 | 2.18× |
| 100 | 1 | 0.041 | 0.128 | 3.14× |
| 100 | 5 | 0.181 | 0.665 | 3.67× |
| 100 | 10 | 0.374 | 1.369 | 3.66× |
| 1,000 | 1 | 0.048 | 0.602 | 12.63× |
| 1,000 | 5 | 0.192 | 3.018 | 15.68× |
| 1,000 | 10 | 0.434 | 5.955 | 13.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
