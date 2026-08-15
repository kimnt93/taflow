# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 423.47M | 0.001 | 938.47M | 0.030 | 12.53× | 27.76× |
| 10,000 | 0.007 | 1.35G | 0.004 | 2.58G | 0.034 | 4.65× | 8.87× |
| 100,000 | 0.064 | 1.57G | 0.037 | 2.70G | 0.066 | 1.04× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.111 | 1.40× |
| 1 | 5 | 0.257 | 0.478 | 1.86× |
| 1 | 10 | 0.433 | 0.968 | 2.23× |
| 10 | 1 | 0.046 | 0.086 | 1.87× |
| 10 | 5 | 0.181 | 0.419 | 2.32× |
| 10 | 10 | 0.391 | 0.996 | 2.55× |
| 100 | 1 | 0.046 | 0.090 | 1.96× |
| 100 | 5 | 0.174 | 0.426 | 2.45× |
| 100 | 10 | 0.376 | 0.868 | 2.31× |
| 1,000 | 1 | 0.040 | 0.090 | 2.25× |
| 1,000 | 5 | 0.216 | 0.446 | 2.06× |
| 1,000 | 10 | 0.405 | 0.903 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
