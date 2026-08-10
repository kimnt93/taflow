# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.71M | 0.019 | 53.29M | 0.047 | 2.18× | 2.48× |
| 10,000 | 0.174 | 57.50M | 0.188 | 53.29M | 0.210 | 1.21× | 1.12× |
| 100,000 | 1.767 | 56.60M | 1.721 | 58.12M | 1.868 | 1.06× | 1.09× |
| 1,000,000 | 17.946 | 55.72M | 17.949 | 55.71M | 18.595 | 1.04× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.140 | 0.93× |
| 1 | 5 | 0.312 | 0.457 | 1.46× |
| 1 | 10 | 0.515 | 0.897 | 1.74× |
| 10 | 1 | 0.060 | 0.091 | 1.51× |
| 10 | 5 | 0.247 | 0.419 | 1.70× |
| 10 | 10 | 0.544 | 0.902 | 1.66× |
| 100 | 1 | 0.056 | 0.085 | 1.53× |
| 100 | 5 | 0.257 | 0.426 | 1.66× |
| 100 | 10 | 0.535 | 0.916 | 1.71× |
| 1,000 | 1 | 0.075 | 0.109 | 1.45× |
| 1,000 | 5 | 0.267 | 0.514 | 1.93× |
| 1,000 | 10 | 0.562 | 1.089 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
