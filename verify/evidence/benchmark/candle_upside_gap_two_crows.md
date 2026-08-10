# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.46M | 0.028 | 36.11M | 0.057 | 1.89× | 2.04× |
| 10,000 | 0.173 | 57.71M | 0.143 | 69.73M | 0.137 | 0.79× | 0.96× |
| 100,000 | 1.475 | 67.78M | 1.449 | 69.02M | 1.121 | 0.76× | 0.77× |
| 1,000,000 | 14.973 | 66.79M | 15.770 | 63.41M | 11.101 | 0.74× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.123 | 1.26× |
| 1 | 5 | 0.289 | 0.552 | 1.91× |
| 1 | 10 | 0.699 | 0.999 | 1.43× |
| 10 | 1 | 0.077 | 0.096 | 1.25× |
| 10 | 5 | 0.277 | 0.458 | 1.65× |
| 10 | 10 | 0.749 | 1.075 | 1.44× |
| 100 | 1 | 0.063 | 0.096 | 1.51× |
| 100 | 5 | 0.283 | 0.547 | 1.93× |
| 100 | 10 | 0.709 | 1.222 | 1.72× |
| 1,000 | 1 | 0.087 | 0.123 | 1.41× |
| 1,000 | 5 | 0.348 | 0.625 | 1.80× |
| 1,000 | 10 | 0.711 | 1.269 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
