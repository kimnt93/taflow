# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.127 | 7.90M | 0.119 | 8.38M | 0.038 | 0.30× | 0.32× |
| 10,000 | 1.120 | 8.93M | 1.071 | 9.34M | 0.131 | 0.12× | 0.12× |
| 100,000 | 11.046 | 9.05M | 11.264 | 8.88M | 1.036 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.116 | 0.99× |
| 1 | 5 | 0.482 | 0.499 | 1.04× |
| 1 | 10 | 0.661 | 0.961 | 1.45× |
| 10 | 1 | 0.072 | 0.092 | 1.28× |
| 10 | 5 | 0.309 | 0.449 | 1.45× |
| 10 | 10 | 0.621 | 0.935 | 1.51× |
| 100 | 1 | 0.082 | 0.094 | 1.15× |
| 100 | 5 | 0.303 | 0.447 | 1.47× |
| 100 | 10 | 0.631 | 0.971 | 1.54× |
| 1,000 | 1 | 0.182 | 0.115 | 0.63× |
| 1,000 | 5 | 0.388 | 0.505 | 1.30× |
| 1,000 | 10 | 0.705 | 1.066 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
