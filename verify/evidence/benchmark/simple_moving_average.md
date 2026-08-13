# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 36.25M | 0.023 | 44.04M | 0.034 | 1.25× | 1.51× |
| 10,000 | 0.156 | 64.27M | 0.144 | 69.61M | 0.050 | 0.32× | 0.34× |
| 100,000 | 1.433 | 69.79M | 1.377 | 72.63M | 0.213 | 0.15× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.109 | 1.31× |
| 1 | 5 | 0.406 | 0.500 | 1.23× |
| 1 | 10 | 0.589 | 0.944 | 1.60× |
| 10 | 1 | 0.064 | 0.093 | 1.45× |
| 10 | 5 | 0.300 | 0.445 | 1.48× |
| 10 | 10 | 0.599 | 0.939 | 1.57× |
| 100 | 1 | 0.067 | 0.086 | 1.29× |
| 100 | 5 | 0.315 | 0.434 | 1.38× |
| 100 | 10 | 0.598 | 0.962 | 1.61× |
| 1,000 | 1 | 0.086 | 0.094 | 1.09× |
| 1,000 | 5 | 0.307 | 0.453 | 1.48× |
| 1,000 | 10 | 0.624 | 0.945 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
