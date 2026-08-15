# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.79M | 0.003 | 298.41M | 0.041 | 6.18× | 12.31× |
| 10,000 | 0.068 | 147.74M | 0.066 | 151.81M | 0.187 | 2.76× | 2.83× |
| 100,000 | 0.925 | 108.06M | 0.883 | 113.23M | 1.594 | 1.72× | 1.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.137 | 1.43× |
| 1 | 5 | 0.228 | 0.476 | 2.08× |
| 1 | 10 | 0.489 | 1.034 | 2.12× |
| 10 | 1 | 0.046 | 0.097 | 2.12× |
| 10 | 5 | 0.180 | 0.429 | 2.38× |
| 10 | 10 | 0.389 | 1.049 | 2.70× |
| 100 | 1 | 0.044 | 0.089 | 2.03× |
| 100 | 5 | 0.195 | 0.466 | 2.39× |
| 100 | 10 | 0.405 | 0.954 | 2.36× |
| 1,000 | 1 | 0.052 | 0.103 | 1.98× |
| 1,000 | 5 | 1.068 | 0.571 | 0.53× |
| 1,000 | 10 | 0.470 | 1.133 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
