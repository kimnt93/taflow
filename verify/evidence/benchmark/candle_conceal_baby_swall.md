# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 54.01M | 0.016 | 63.44M | 0.033 | 1.79× | 2.10× |
| 10,000 | 0.124 | 80.76M | 0.122 | 81.81M | 0.091 | 0.73× | 0.74× |
| 100,000 | 1.228 | 81.42M | 1.214 | 82.35M | 0.649 | 0.53× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.137 | 1.14× |
| 1 | 5 | 0.304 | 0.443 | 1.46× |
| 1 | 10 | 0.515 | 0.866 | 1.68× |
| 10 | 1 | 0.062 | 0.087 | 1.40× |
| 10 | 5 | 0.267 | 0.474 | 1.78× |
| 10 | 10 | 0.522 | 0.902 | 1.73× |
| 100 | 1 | 0.061 | 0.091 | 1.48× |
| 100 | 5 | 0.261 | 0.424 | 1.62× |
| 100 | 10 | 0.593 | 0.923 | 1.56× |
| 1,000 | 1 | 0.065 | 0.100 | 1.54× |
| 1,000 | 5 | 0.269 | 0.467 | 1.73× |
| 1,000 | 10 | 0.550 | 1.041 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
