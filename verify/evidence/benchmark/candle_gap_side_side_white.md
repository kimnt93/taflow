# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.110 | 9.12M | 0.101 | 9.93M | 0.045 | 0.41× | 0.44× |
| 10,000 | 0.924 | 10.83M | 0.890 | 11.24M | 0.219 | 0.24× | 0.25× |
| 100,000 | 9.519 | 10.51M | 8.515 | 11.74M | 2.042 | 0.21× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.154 | 0.98× |
| 1 | 5 | 0.394 | 0.464 | 1.18× |
| 1 | 10 | 0.624 | 0.930 | 1.49× |
| 10 | 1 | 0.067 | 0.091 | 1.35× |
| 10 | 5 | 0.309 | 0.465 | 1.50× |
| 10 | 10 | 0.607 | 0.940 | 1.55× |
| 100 | 1 | 0.076 | 0.105 | 1.37× |
| 100 | 5 | 0.323 | 0.446 | 1.38× |
| 100 | 10 | 0.634 | 0.947 | 1.49× |
| 1,000 | 1 | 0.169 | 0.115 | 0.68× |
| 1,000 | 5 | 0.349 | 0.538 | 1.54× |
| 1,000 | 10 | 0.683 | 1.107 | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
