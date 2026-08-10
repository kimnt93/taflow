# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.99M | 0.014 | 70.42M | 0.036 | 2.06× | 2.54× |
| 10,000 | 0.152 | 65.99M | 0.138 | 72.71M | 0.127 | 0.84× | 0.92× |
| 100,000 | 1.444 | 69.26M | 1.619 | 61.76M | 1.047 | 0.72× | 0.65× |
| 1,000,000 | 15.146 | 66.03M | 15.295 | 65.38M | 9.763 | 0.64× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.106 | 1.04× |
| 1 | 5 | 0.321 | 0.453 | 1.41× |
| 1 | 10 | 0.538 | 0.919 | 1.71× |
| 10 | 1 | 0.057 | 0.096 | 1.67× |
| 10 | 5 | 0.249 | 0.429 | 1.72× |
| 10 | 10 | 0.558 | 0.884 | 1.58× |
| 100 | 1 | 0.058 | 0.086 | 1.50× |
| 100 | 5 | 0.259 | 0.438 | 1.69× |
| 100 | 10 | 0.537 | 0.900 | 1.68× |
| 1,000 | 1 | 0.068 | 0.102 | 1.50× |
| 1,000 | 5 | 0.266 | 0.469 | 1.76× |
| 1,000 | 10 | 0.594 | 1.021 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
