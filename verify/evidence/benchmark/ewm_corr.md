# ExponentiallyWeightedCorrelation benchmark (`ewm correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.04M | 0.006 | 155.22M | 1.295 | 161.90× | 200.97× |
| 10,000 | 0.056 | 178.55M | 0.054 | 185.48M | 12.783 | 228.24× | 237.10× |
| 100,000 | 0.554 | 180.51M | 0.494 | 202.35M | 122.848 | 221.76× | 248.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.146 | 1.48× |
| 1 | 5 | 0.251 | 0.521 | 2.07× |
| 1 | 10 | 0.415 | 1.098 | 2.64× |
| 10 | 1 | 0.052 | 0.116 | 2.24× |
| 10 | 5 | 0.202 | 0.578 | 2.85× |
| 10 | 10 | 0.396 | 1.134 | 2.86× |
| 100 | 1 | 0.044 | 0.227 | 5.19× |
| 100 | 5 | 0.214 | 1.126 | 5.27× |
| 100 | 10 | 0.415 | 2.296 | 5.53× |
| 1,000 | 1 | 0.057 | 1.366 | 23.97× |
| 1,000 | 5 | 0.235 | 6.963 | 29.67× |
| 1,000 | 10 | 0.429 | 13.988 | 32.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
