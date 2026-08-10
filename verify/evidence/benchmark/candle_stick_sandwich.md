# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.32M | 0.010 | 102.10M | 0.042 | 2.96× | 4.24× |
| 10,000 | 0.066 | 151.77M | 0.061 | 164.85M | 0.107 | 1.63× | 1.77× |
| 100,000 | 0.733 | 136.51M | 0.719 | 139.00M | 0.792 | 1.08× | 1.10× |
| 1,000,000 | 7.690 | 130.04M | 7.358 | 135.90M | 7.423 | 0.97× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.143 | 1.66× |
| 1 | 5 | 0.376 | 0.595 | 1.58× |
| 1 | 10 | 0.752 | 1.296 | 1.72× |
| 10 | 1 | 0.070 | 0.129 | 1.85× |
| 10 | 5 | 0.343 | 0.538 | 1.57× |
| 10 | 10 | 0.943 | 1.241 | 1.32× |
| 100 | 1 | 0.072 | 0.107 | 1.49× |
| 100 | 5 | 0.415 | 0.668 | 1.61× |
| 100 | 10 | 0.719 | 1.233 | 1.71× |
| 1,000 | 1 | 0.079 | 0.121 | 1.52× |
| 1,000 | 5 | 0.433 | 0.629 | 1.45× |
| 1,000 | 10 | 0.738 | 1.348 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
