# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.80M | 0.008 | 122.21M | 0.041 | 4.25× | 5.05× |
| 10,000 | 0.078 | 127.88M | 0.073 | 137.44M | 0.128 | 1.63× | 1.75× |
| 100,000 | 0.750 | 133.42M | 0.733 | 136.44M | 0.957 | 1.28× | 1.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.173 | 1.75× |
| 1 | 5 | 0.264 | 0.502 | 1.90× |
| 1 | 10 | 0.386 | 1.002 | 2.60× |
| 10 | 1 | 0.045 | 0.094 | 2.09× |
| 10 | 5 | 0.211 | 0.489 | 2.32× |
| 10 | 10 | 0.411 | 0.981 | 2.39× |
| 100 | 1 | 0.042 | 0.104 | 2.50× |
| 100 | 5 | 0.190 | 0.455 | 2.40× |
| 100 | 10 | 0.430 | 1.005 | 2.34× |
| 1,000 | 1 | 0.055 | 0.099 | 1.81× |
| 1,000 | 5 | 0.195 | 0.499 | 2.56× |
| 1,000 | 10 | 0.427 | 1.159 | 2.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
