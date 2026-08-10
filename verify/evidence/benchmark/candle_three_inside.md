# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.14M | 0.012 | 83.87M | 0.041 | 2.92× | 3.44× |
| 10,000 | 0.110 | 90.81M | 0.102 | 97.67M | 0.137 | 1.24× | 1.34× |
| 100,000 | 1.146 | 87.29M | 1.170 | 85.44M | 1.232 | 1.08× | 1.05× |
| 1,000,000 | 13.176 | 75.90M | 11.326 | 88.29M | 11.509 | 0.87× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.131 | 1.84× |
| 1 | 5 | 0.326 | 0.543 | 1.67× |
| 1 | 10 | 0.582 | 1.000 | 1.72× |
| 10 | 1 | 0.056 | 0.091 | 1.61× |
| 10 | 5 | 0.258 | 0.501 | 1.94× |
| 10 | 10 | 0.602 | 0.977 | 1.62× |
| 100 | 1 | 0.059 | 0.094 | 1.59× |
| 100 | 5 | 0.327 | 0.464 | 1.42× |
| 100 | 10 | 0.627 | 1.007 | 1.60× |
| 1,000 | 1 | 0.067 | 0.108 | 1.60× |
| 1,000 | 5 | 0.273 | 0.484 | 1.77× |
| 1,000 | 10 | 0.548 | 1.142 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
