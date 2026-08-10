# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.17M | 0.016 | 61.64M | 0.039 | 2.00× | 2.41× |
| 10,000 | 0.161 | 62.30M | 0.165 | 60.63M | 0.182 | 1.13× | 1.10× |
| 100,000 | 1.591 | 62.86M | 1.521 | 65.75M | 1.466 | 0.92× | 0.96× |
| 1,000,000 | 15.703 | 63.68M | 15.769 | 63.42M | 14.860 | 0.95× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.142 | 1.32× |
| 1 | 5 | 0.403 | 0.490 | 1.22× |
| 1 | 10 | 0.550 | 0.911 | 1.66× |
| 10 | 1 | 0.053 | 0.090 | 1.70× |
| 10 | 5 | 0.244 | 0.421 | 1.73× |
| 10 | 10 | 0.530 | 0.913 | 1.72× |
| 100 | 1 | 0.061 | 0.094 | 1.53× |
| 100 | 5 | 0.260 | 0.429 | 1.65× |
| 100 | 10 | 0.539 | 0.916 | 1.70× |
| 1,000 | 1 | 0.067 | 0.107 | 1.61× |
| 1,000 | 5 | 0.271 | 0.504 | 1.86× |
| 1,000 | 10 | 0.561 | 1.060 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
