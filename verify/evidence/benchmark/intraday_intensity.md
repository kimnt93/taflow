# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.56M | 0.004 | 251.50M | 0.220 | 31.39× | 55.37× |
| 10,000 | 0.036 | 280.34M | 0.031 | 325.60M | 1.332 | 37.35× | 43.38× |
| 100,000 | 0.290 | 345.34M | 0.276 | 361.84M | 12.417 | 42.88× | 44.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.210 | 3.77× |
| 1 | 5 | 0.253 | 0.838 | 3.32× |
| 1 | 10 | 0.423 | 1.654 | 3.91× |
| 10 | 1 | 0.044 | 0.167 | 3.80× |
| 10 | 5 | 0.211 | 1.227 | 5.80× |
| 10 | 10 | 0.408 | 1.653 | 4.05× |
| 100 | 1 | 0.052 | 0.186 | 3.57× |
| 100 | 5 | 0.211 | 1.153 | 5.47× |
| 100 | 10 | 0.405 | 1.773 | 4.37× |
| 1,000 | 1 | 0.053 | 0.313 | 5.95× |
| 1,000 | 5 | 0.219 | 1.698 | 7.77× |
| 1,000 | 10 | 0.416 | 3.048 | 7.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
