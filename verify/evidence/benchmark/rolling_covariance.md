# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.56M | 0.010 | 97.02M | 0.217 | 18.37× | 21.08× |
| 10,000 | 0.099 | 100.97M | 0.092 | 108.64M | 0.838 | 8.47× | 9.11× |
| 100,000 | 0.970 | 103.12M | 0.952 | 105.03M | 8.064 | 8.32× | 8.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.252 | 3.71× |
| 1 | 5 | 0.242 | 1.100 | 4.55× |
| 1 | 10 | 0.422 | 2.216 | 5.25× |
| 10 | 1 | 0.044 | 0.213 | 4.79× |
| 10 | 5 | 0.216 | 1.375 | 6.38× |
| 10 | 10 | 0.413 | 2.314 | 5.60× |
| 100 | 1 | 0.047 | 0.233 | 4.96× |
| 100 | 5 | 0.214 | 1.324 | 6.19× |
| 100 | 10 | 0.424 | 2.394 | 5.64× |
| 1,000 | 1 | 0.062 | 0.284 | 4.56× |
| 1,000 | 5 | 0.213 | 1.599 | 7.51× |
| 1,000 | 10 | 0.450 | 3.056 | 6.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
