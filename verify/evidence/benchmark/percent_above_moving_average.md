# PercentAboveMovingAverage benchmark (`PercentAboveMa` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.53M | 0.006 | 181.51M | 11.570 | 1695.41× | 2100.15× |
| 10,000 | 0.031 | 327.52M | 0.027 | 376.31M | 114.622 | 3754.09× | 4313.38× |
| 100,000 | 0.246 | 406.49M | 0.221 | 452.23M | 1138.115 | 4626.29× | 5146.90× |
| 1,000,000 | 3.032 | 329.76M | 2.473 | 404.30M | 11491.585 | 3789.50× | 4646.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.246 | 3.50× |
| 1 | 5 | 0.238 | 1.063 | 4.47× |
| 1 | 10 | 0.517 | 2.413 | 4.67× |
| 10 | 1 | 0.055 | 0.328 | 5.96× |
| 10 | 5 | 0.240 | 1.811 | 7.54× |
| 10 | 10 | 0.471 | 3.160 | 6.71× |
| 100 | 1 | 0.049 | 1.364 | 27.77× |
| 100 | 5 | 0.231 | 7.081 | 30.63× |
| 100 | 10 | 0.473 | 14.049 | 29.69× |
| 1,000 | 1 | 0.072 | 11.609 | 160.68× |
| 1,000 | 5 | 0.364 | 62.647 | 171.95× |
| 1,000 | 10 | 0.628 | 116.898 | 186.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
