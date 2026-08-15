# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.76M | 0.003 | 293.56M | 0.035 | 5.31× | 10.41× |
| 10,000 | 0.045 | 222.33M | 0.041 | 245.72M | 0.120 | 2.68× | 2.96× |
| 100,000 | 0.624 | 160.29M | 0.611 | 163.78M | 0.946 | 1.52× | 1.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.118 | 1.56× |
| 1 | 5 | 0.311 | 0.457 | 1.47× |
| 1 | 10 | 0.394 | 0.916 | 2.32× |
| 10 | 1 | 0.043 | 0.090 | 2.11× |
| 10 | 5 | 0.179 | 0.432 | 2.41× |
| 10 | 10 | 0.379 | 0.878 | 2.32× |
| 100 | 1 | 0.042 | 0.090 | 2.15× |
| 100 | 5 | 0.199 | 0.437 | 2.19× |
| 100 | 10 | 0.388 | 0.893 | 2.30× |
| 1,000 | 1 | 0.049 | 0.096 | 1.97× |
| 1,000 | 5 | 0.202 | 0.472 | 2.34× |
| 1,000 | 10 | 0.407 | 0.971 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
