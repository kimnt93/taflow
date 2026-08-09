# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.35M | 0.007 | 141.38M | 0.030 | 3.33× | 4.31× |
| 10,000 | 0.063 | 159.78M | 0.059 | 168.91M | 0.077 | 1.22× | 1.29× |
| 100,000 | 0.900 | 111.11M | 0.858 | 116.56M | 0.554 | 0.62× | 0.65× |
| 1,000,000 | 8.844 | 113.06M | 8.803 | 113.59M | 5.627 | 0.64× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.102 | 1.39× |
| 1 | 5 | 0.344 | 0.540 | 1.57× |
| 1 | 10 | 0.541 | 0.960 | 1.78× |
| 10 | 1 | 0.056 | 0.096 | 1.72× |
| 10 | 5 | 0.253 | 0.455 | 1.80× |
| 10 | 10 | 0.541 | 0.989 | 1.83× |
| 100 | 1 | 0.060 | 0.091 | 1.53× |
| 100 | 5 | 0.269 | 0.452 | 1.68× |
| 100 | 10 | 0.549 | 0.977 | 1.78× |
| 1,000 | 1 | 0.064 | 0.102 | 1.58× |
| 1,000 | 5 | 0.269 | 0.484 | 1.80× |
| 1,000 | 10 | 0.566 | 1.077 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
