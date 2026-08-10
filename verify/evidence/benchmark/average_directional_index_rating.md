# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.46M | 0.017 | 57.64M | 0.043 | 2.45× | 2.46× |
| 10,000 | 0.114 | 87.51M | 0.115 | 86.75M | 0.135 | 1.18× | 1.17× |
| 100,000 | 1.051 | 95.17M | 1.016 | 98.38M | 1.036 | 0.99× | 1.02× |
| 1,000,000 | 10.703 | 93.43M | 10.899 | 91.75M | 11.870 | 1.11× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.136 | 1.35× |
| 1 | 5 | 0.389 | 0.463 | 1.19× |
| 1 | 10 | 0.505 | 0.935 | 1.85× |
| 10 | 1 | 0.055 | 0.090 | 1.63× |
| 10 | 5 | 0.236 | 0.435 | 1.84× |
| 10 | 10 | 0.525 | 0.977 | 1.86× |
| 100 | 1 | 0.065 | 0.096 | 1.49× |
| 100 | 5 | 0.246 | 0.461 | 1.88× |
| 100 | 10 | 0.541 | 0.974 | 1.80× |
| 1,000 | 1 | 0.066 | 0.104 | 1.56× |
| 1,000 | 5 | 0.259 | 0.500 | 1.93× |
| 1,000 | 10 | 0.546 | 1.084 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
