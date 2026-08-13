# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.108 | 9.24M | 0.095 | 10.53M | 0.034 | 0.31× | 0.36× |
| 10,000 | 0.902 | 11.09M | 0.876 | 11.42M | 0.136 | 0.15× | 0.16× |
| 100,000 | 8.683 | 11.52M | 8.522 | 11.73M | 1.147 | 0.13× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.106 | 0.76× |
| 1 | 5 | 0.421 | 0.453 | 1.08× |
| 1 | 10 | 0.674 | 1.006 | 1.49× |
| 10 | 1 | 0.093 | 0.099 | 1.07× |
| 10 | 5 | 0.317 | 0.444 | 1.40× |
| 10 | 10 | 0.645 | 0.913 | 1.42× |
| 100 | 1 | 0.076 | 0.101 | 1.34× |
| 100 | 5 | 0.317 | 0.454 | 1.43× |
| 100 | 10 | 0.691 | 0.928 | 1.34× |
| 1,000 | 1 | 0.160 | 0.101 | 0.63× |
| 1,000 | 5 | 0.350 | 0.496 | 1.42× |
| 1,000 | 10 | 0.696 | 1.053 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
