# TomDeMarkSequential benchmark (`TDSequential` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.26M | 0.007 | 139.14M | 0.616 | 72.22× | 85.69× |
| 10,000 | 0.079 | 127.26M | 0.073 | 137.31M | 4.474 | 56.94× | 61.44× |
| 100,000 | 0.768 | 130.23M | 0.744 | 134.43M | 48.024 | 62.54× | 64.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.370 | 4.76× |
| 1 | 5 | 0.238 | 1.504 | 6.33× |
| 1 | 10 | 0.405 | 3.155 | 7.80× |
| 10 | 1 | 0.047 | 0.267 | 5.65× |
| 10 | 5 | 0.193 | 1.463 | 7.58× |
| 10 | 10 | 0.408 | 3.161 | 7.74× |
| 100 | 1 | 0.046 | 0.302 | 6.58× |
| 100 | 5 | 0.186 | 1.719 | 9.24× |
| 100 | 10 | 0.412 | 3.583 | 8.71× |
| 1,000 | 1 | 0.064 | 0.828 | 12.93× |
| 1,000 | 5 | 0.227 | 4.140 | 18.26× |
| 1,000 | 10 | 0.527 | 8.259 | 15.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
