# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.58M | 0.017 | 59.29M | 0.038 | 1.89× | 2.25× |
| 10,000 | 0.144 | 69.48M | 0.143 | 69.81M | 0.120 | 0.83× | 0.84× |
| 100,000 | 1.493 | 66.97M | 1.472 | 67.91M | 0.901 | 0.60× | 0.61× |
| 1,000,000 | 14.526 | 68.84M | 14.016 | 71.35M | 8.822 | 0.61× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.119 | 1.48× |
| 1 | 5 | 0.256 | 0.474 | 1.85× |
| 1 | 10 | 0.500 | 1.002 | 2.01× |
| 10 | 1 | 0.062 | 0.109 | 1.77× |
| 10 | 5 | 0.293 | 0.502 | 1.71× |
| 10 | 10 | 0.610 | 0.977 | 1.60× |
| 100 | 1 | 0.059 | 0.100 | 1.71× |
| 100 | 5 | 0.257 | 0.546 | 2.12× |
| 100 | 10 | 0.620 | 1.017 | 1.64× |
| 1,000 | 1 | 0.077 | 0.107 | 1.38× |
| 1,000 | 5 | 0.282 | 0.550 | 1.95× |
| 1,000 | 10 | 0.669 | 1.379 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
