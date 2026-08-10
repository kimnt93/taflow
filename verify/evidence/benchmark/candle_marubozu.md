# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.41M | 0.016 | 64.19M | 0.034 | 1.90× | 2.20× |
| 10,000 | 0.146 | 68.49M | 0.130 | 76.80M | 0.136 | 0.93× | 1.04× |
| 100,000 | 1.350 | 74.07M | 1.295 | 77.20M | 1.016 | 0.75× | 0.78× |
| 1,000,000 | 14.212 | 70.36M | 14.180 | 70.52M | 10.227 | 0.72× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.110 | 0.61× |
| 1 | 5 | 0.284 | 0.471 | 1.66× |
| 1 | 10 | 0.549 | 0.950 | 1.73× |
| 10 | 1 | 0.065 | 0.095 | 1.46× |
| 10 | 5 | 0.257 | 0.429 | 1.67× |
| 10 | 10 | 0.553 | 0.979 | 1.77× |
| 100 | 1 | 0.063 | 0.086 | 1.36× |
| 100 | 5 | 0.251 | 0.425 | 1.69× |
| 100 | 10 | 0.552 | 0.921 | 1.67× |
| 1,000 | 1 | 0.071 | 0.103 | 1.46× |
| 1,000 | 5 | 0.270 | 0.488 | 1.81× |
| 1,000 | 10 | 0.602 | 1.025 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
