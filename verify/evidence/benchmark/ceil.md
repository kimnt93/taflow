# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.23M | 0.020 | 50.16M | 0.027 | 1.10× | 1.37× |
| 10,000 | 0.146 | 68.42M | 0.139 | 72.05M | 0.040 | 0.28× | 0.29× |
| 100,000 | 1.357 | 73.68M | 1.326 | 75.40M | 0.163 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.100 | 1.16× |
| 1 | 5 | 0.395 | 0.465 | 1.18× |
| 1 | 10 | 0.594 | 0.855 | 1.44× |
| 10 | 1 | 0.059 | 0.089 | 1.49× |
| 10 | 5 | 0.269 | 0.410 | 1.52× |
| 10 | 10 | 0.592 | 0.900 | 1.52× |
| 100 | 1 | 0.063 | 0.083 | 1.32× |
| 100 | 5 | 0.279 | 0.405 | 1.45× |
| 100 | 10 | 0.574 | 0.870 | 1.52× |
| 1,000 | 1 | 0.081 | 0.096 | 1.18× |
| 1,000 | 5 | 0.290 | 0.407 | 1.40× |
| 1,000 | 10 | 0.608 | 0.873 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
