# RollingSkew benchmark (`Skewness` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.90M | 0.038 | 26.50M | 0.214 | 5.97× | 5.67× |
| 10,000 | 0.337 | 29.71M | 0.337 | 29.70M | 0.737 | 2.19× | 2.19× |
| 100,000 | 3.394 | 29.47M | 3.488 | 28.67M | 6.810 | 2.01× | 1.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.253 | 2.53× |
| 1 | 5 | 0.301 | 1.099 | 3.65× |
| 1 | 10 | 0.477 | 2.366 | 4.96× |
| 10 | 1 | 0.053 | 0.209 | 3.91× |
| 10 | 5 | 0.231 | 1.257 | 5.44× |
| 10 | 10 | 0.511 | 2.299 | 4.49× |
| 100 | 1 | 0.056 | 0.220 | 3.93× |
| 100 | 5 | 0.251 | 1.443 | 5.74× |
| 100 | 10 | 0.534 | 2.406 | 4.50× |
| 1,000 | 1 | 0.090 | 0.292 | 3.25× |
| 1,000 | 5 | 0.280 | 1.655 | 5.91× |
| 1,000 | 10 | 0.522 | 3.181 | 6.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
