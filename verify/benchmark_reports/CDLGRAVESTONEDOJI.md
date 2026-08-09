# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.56M | 0.008 | 132.55M | 0.034 | 3.92× | 4.46× |
| 10,000 | 0.051 | 194.77M | 0.050 | 200.00M | 0.098 | 1.90× | 1.95× |
| 100,000 | 0.550 | 181.87M | 0.535 | 186.94M | 0.776 | 1.41× | 1.45× |
| 1,000,000 | 5.988 | 167.00M | 5.770 | 173.32M | 7.388 | 1.23× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.122 | 0.95× |
| 1 | 5 | 0.291 | 0.483 | 1.66× |
| 1 | 10 | 0.570 | 1.030 | 1.81× |
| 10 | 1 | 0.055 | 0.099 | 1.79× |
| 10 | 5 | 0.250 | 0.481 | 1.92× |
| 10 | 10 | 0.544 | 0.979 | 1.80× |
| 100 | 1 | 0.062 | 0.094 | 1.52× |
| 100 | 5 | 0.268 | 0.471 | 1.76× |
| 100 | 10 | 0.527 | 0.977 | 1.86× |
| 1,000 | 1 | 0.061 | 0.098 | 1.61× |
| 1,000 | 5 | 0.328 | 0.520 | 1.59× |
| 1,000 | 10 | 0.556 | 1.084 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
