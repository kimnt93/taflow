# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.12M | 0.004 | 243.36M | 0.039 | 7.62× | 9.50× |
| 10,000 | 0.035 | 284.83M | 0.033 | 305.99M | 0.093 | 2.65× | 2.85× |
| 100,000 | 0.334 | 299.49M | 0.315 | 316.97M | 0.930 | 2.79× | 2.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.109 | 1.95× |
| 1 | 5 | 0.334 | 0.523 | 1.56× |
| 1 | 10 | 0.409 | 0.941 | 2.30× |
| 10 | 1 | 0.042 | 0.092 | 2.21× |
| 10 | 5 | 0.179 | 0.445 | 2.49× |
| 10 | 10 | 0.372 | 1.008 | 2.71× |
| 100 | 1 | 0.050 | 0.093 | 1.87× |
| 100 | 5 | 0.201 | 0.440 | 2.19× |
| 100 | 10 | 0.381 | 0.944 | 2.48× |
| 1,000 | 1 | 0.063 | 0.109 | 1.73× |
| 1,000 | 5 | 0.201 | 0.609 | 3.03× |
| 1,000 | 10 | 0.439 | 1.022 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
