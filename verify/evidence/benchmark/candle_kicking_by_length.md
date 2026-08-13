# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.144 | 6.96M | 0.135 | 7.40M | 0.038 | 0.27× | 0.28× |
| 10,000 | 1.363 | 7.34M | 1.275 | 7.84M | 0.166 | 0.12× | 0.13× |
| 100,000 | 12.425 | 8.05M | 13.855 | 7.22M | 1.385 | 0.11× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.105 | 0.90× |
| 1 | 5 | 0.463 | 0.451 | 0.97× |
| 1 | 10 | 0.671 | 0.970 | 1.45× |
| 10 | 1 | 0.082 | 0.101 | 1.23× |
| 10 | 5 | 0.303 | 0.441 | 1.45× |
| 10 | 10 | 0.660 | 0.919 | 1.39× |
| 100 | 1 | 0.084 | 0.088 | 1.04× |
| 100 | 5 | 0.302 | 0.441 | 1.46× |
| 100 | 10 | 0.653 | 0.928 | 1.42× |
| 1,000 | 1 | 0.203 | 0.113 | 0.56× |
| 1,000 | 5 | 0.410 | 0.514 | 1.25× |
| 1,000 | 10 | 0.764 | 1.045 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
