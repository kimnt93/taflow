# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.37M | 0.043 | 23.09M | 0.033 | 0.67× | 0.76× |
| 10,000 | 0.387 | 25.87M | 0.352 | 28.39M | 0.058 | 0.15× | 0.17× |
| 100,000 | 3.613 | 27.68M | 3.588 | 27.87M | 0.310 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.124 | 1.13× |
| 1 | 5 | 0.459 | 0.497 | 1.08× |
| 1 | 10 | 0.613 | 0.945 | 1.54× |
| 10 | 1 | 0.069 | 0.090 | 1.31× |
| 10 | 5 | 0.306 | 0.482 | 1.58× |
| 10 | 10 | 0.633 | 0.943 | 1.49× |
| 100 | 1 | 0.068 | 0.090 | 1.32× |
| 100 | 5 | 0.293 | 0.448 | 1.53× |
| 100 | 10 | 0.656 | 0.944 | 1.44× |
| 1,000 | 1 | 0.101 | 0.100 | 0.99× |
| 1,000 | 5 | 0.302 | 0.459 | 1.52× |
| 1,000 | 10 | 0.708 | 0.998 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
