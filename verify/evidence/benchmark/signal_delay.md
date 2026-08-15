# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 538.87M | 0.001 | 1.04G | 0.024 | 12.78× | 24.67× |
| 10,000 | 0.007 | 1.40G | 0.004 | 2.27G | 0.028 | 3.96× | 6.41× |
| 100,000 | 0.068 | 1.47G | 0.044 | 2.28G | 0.069 | 1.02× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.090 | 1.54× |
| 1 | 5 | 0.306 | 0.424 | 1.38× |
| 1 | 10 | 0.390 | 0.805 | 2.07× |
| 10 | 1 | 0.042 | 0.084 | 1.97× |
| 10 | 5 | 0.175 | 0.410 | 2.35× |
| 10 | 10 | 0.375 | 0.863 | 2.30× |
| 100 | 1 | 0.040 | 0.087 | 2.18× |
| 100 | 5 | 0.180 | 0.393 | 2.18× |
| 100 | 10 | 0.386 | 0.850 | 2.20× |
| 1,000 | 1 | 0.040 | 0.084 | 2.09× |
| 1,000 | 5 | 0.168 | 0.417 | 2.48× |
| 1,000 | 10 | 0.365 | 0.884 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
