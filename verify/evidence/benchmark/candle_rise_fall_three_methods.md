# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.28M | 0.016 | 61.12M | 0.035 | 1.77× | 2.16× |
| 10,000 | 0.184 | 54.40M | 0.172 | 58.06M | 0.118 | 0.64× | 0.68× |
| 100,000 | 1.890 | 52.92M | 1.782 | 56.11M | 0.905 | 0.48× | 0.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.115 | 1.11× |
| 1 | 5 | 0.242 | 0.472 | 1.94× |
| 1 | 10 | 0.403 | 0.911 | 2.26× |
| 10 | 1 | 0.041 | 0.087 | 2.11× |
| 10 | 5 | 0.182 | 0.474 | 2.61× |
| 10 | 10 | 0.406 | 0.880 | 2.17× |
| 100 | 1 | 0.044 | 0.086 | 1.97× |
| 100 | 5 | 0.182 | 0.421 | 2.31× |
| 100 | 10 | 0.412 | 0.990 | 2.40× |
| 1,000 | 1 | 0.062 | 0.099 | 1.60× |
| 1,000 | 5 | 0.199 | 0.466 | 2.34× |
| 1,000 | 10 | 0.425 | 0.985 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
