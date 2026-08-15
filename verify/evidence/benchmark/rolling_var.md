# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.43M | 0.004 | 239.52M | 0.036 | 6.77× | 8.70× |
| 10,000 | 0.036 | 274.87M | 0.034 | 296.79M | 0.053 | 1.47× | 1.58× |
| 100,000 | 0.348 | 287.71M | 0.318 | 314.13M | 0.261 | 0.75× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.107 | 1.57× |
| 1 | 5 | 0.221 | 0.508 | 2.30× |
| 1 | 10 | 0.400 | 1.020 | 2.55× |
| 10 | 1 | 0.047 | 0.107 | 2.26× |
| 10 | 5 | 0.202 | 0.469 | 2.32× |
| 10 | 10 | 0.396 | 0.961 | 2.43× |
| 100 | 1 | 0.045 | 0.100 | 2.21× |
| 100 | 5 | 0.217 | 0.460 | 2.12× |
| 100 | 10 | 0.393 | 0.912 | 2.32× |
| 1,000 | 1 | 0.050 | 0.092 | 1.84× |
| 1,000 | 5 | 0.197 | 0.484 | 2.46× |
| 1,000 | 10 | 0.511 | 1.021 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
