# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.48M | 0.005 | 220.63M | 0.039 | 5.04× | 8.65× |
| 10,000 | 0.096 | 104.29M | 0.092 | 108.52M | 0.137 | 1.43× | 1.49× |
| 100,000 | 1.007 | 99.27M | 1.027 | 97.33M | 1.105 | 1.10× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.100 | 0.63× |
| 1 | 5 | 0.199 | 0.439 | 2.20× |
| 1 | 10 | 0.397 | 0.895 | 2.25× |
| 10 | 1 | 0.039 | 0.092 | 2.35× |
| 10 | 5 | 0.198 | 0.476 | 2.40× |
| 10 | 10 | 0.394 | 0.890 | 2.26× |
| 100 | 1 | 0.045 | 0.087 | 1.95× |
| 100 | 5 | 0.211 | 0.432 | 2.05× |
| 100 | 10 | 0.455 | 0.948 | 2.09× |
| 1,000 | 1 | 0.056 | 0.096 | 1.71× |
| 1,000 | 5 | 0.209 | 0.485 | 2.32× |
| 1,000 | 10 | 0.429 | 1.067 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
