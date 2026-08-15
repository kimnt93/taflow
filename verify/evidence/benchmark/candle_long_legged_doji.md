# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.61M | 0.003 | 371.20M | 0.034 | 5.61× | 12.80× |
| 10,000 | 0.045 | 222.78M | 0.043 | 235.13M | 0.090 | 1.99× | 2.11× |
| 100,000 | 0.519 | 192.83M | 0.493 | 202.77M | 0.667 | 1.29× | 1.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.103 | 1.81× |
| 1 | 5 | 0.236 | 0.440 | 1.86× |
| 1 | 10 | 0.426 | 0.930 | 2.18× |
| 10 | 1 | 0.042 | 0.088 | 2.09× |
| 10 | 5 | 0.181 | 0.409 | 2.26× |
| 10 | 10 | 0.385 | 0.967 | 2.51× |
| 100 | 1 | 0.042 | 0.097 | 2.32× |
| 100 | 5 | 0.184 | 0.439 | 2.39× |
| 100 | 10 | 0.377 | 0.898 | 2.38× |
| 1,000 | 1 | 0.045 | 0.103 | 2.29× |
| 1,000 | 5 | 0.216 | 0.499 | 2.31× |
| 1,000 | 10 | 0.461 | 0.980 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
