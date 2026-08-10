# RollingCalmar benchmark (`rolling calmar on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.40M | 0.043 | 23.05M | 0.246 | 5.52× | 5.68× |
| 10,000 | 0.449 | 22.29M | 0.426 | 23.47M | 1.450 | 3.23× | 3.40× |
| 100,000 | 4.494 | 22.25M | 4.286 | 23.33M | 16.271 | 3.62× | 3.80× |
| 1,000,000 | 43.224 | 23.14M | 44.823 | 22.31M | 172.748 | 4.00× | 3.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.148 | 1.90× |
| 1 | 5 | 0.292 | 0.453 | 1.55× |
| 1 | 10 | 0.459 | 0.990 | 2.16× |
| 10 | 1 | 0.050 | 0.088 | 1.76× |
| 10 | 5 | 0.222 | 0.413 | 1.86× |
| 10 | 10 | 0.469 | 0.860 | 1.83× |
| 100 | 1 | 0.056 | 0.192 | 3.45× |
| 100 | 5 | 0.229 | 1.037 | 4.53× |
| 100 | 10 | 0.532 | 2.137 | 4.02× |
| 1,000 | 1 | 0.093 | 0.327 | 3.52× |
| 1,000 | 5 | 0.247 | 1.134 | 4.60× |
| 1,000 | 10 | 0.549 | 2.437 | 4.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
