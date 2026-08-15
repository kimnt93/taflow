# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.53M | 0.007 | 137.45M | 0.053 | 4.94× | 7.27× |
| 10,000 | 0.096 | 103.87M | 0.091 | 109.34M | 0.237 | 2.46× | 2.59× |
| 100,000 | 1.055 | 94.78M | 1.035 | 96.64M | 2.183 | 2.07× | 2.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.115 | 1.16× |
| 1 | 5 | 0.272 | 0.622 | 2.29× |
| 1 | 10 | 0.521 | 0.965 | 1.85× |
| 10 | 1 | 0.043 | 0.093 | 2.17× |
| 10 | 5 | 0.198 | 0.442 | 2.23× |
| 10 | 10 | 0.397 | 1.036 | 2.61× |
| 100 | 1 | 0.047 | 0.092 | 1.98× |
| 100 | 5 | 0.183 | 0.430 | 2.35× |
| 100 | 10 | 0.399 | 0.915 | 2.29× |
| 1,000 | 1 | 0.060 | 0.114 | 1.91× |
| 1,000 | 5 | 0.220 | 0.580 | 2.63× |
| 1,000 | 10 | 0.418 | 1.094 | 2.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
