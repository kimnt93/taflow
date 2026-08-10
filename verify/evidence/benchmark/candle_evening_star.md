# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.16M | 0.018 | 55.27M | 0.041 | 1.87× | 2.29× |
| 10,000 | 0.164 | 60.87M | 0.154 | 64.87M | 0.122 | 0.74× | 0.79× |
| 100,000 | 1.518 | 65.86M | 1.575 | 63.50M | 0.916 | 0.60× | 0.58× |
| 1,000,000 | 15.821 | 63.21M | 14.718 | 67.94M | 8.793 | 0.56× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.138 | 1.71× |
| 1 | 5 | 0.318 | 0.464 | 1.46× |
| 1 | 10 | 0.515 | 0.995 | 1.93× |
| 10 | 1 | 0.062 | 0.101 | 1.61× |
| 10 | 5 | 0.287 | 0.477 | 1.66× |
| 10 | 10 | 0.554 | 1.027 | 1.85× |
| 100 | 1 | 0.064 | 0.104 | 1.62× |
| 100 | 5 | 0.276 | 0.476 | 1.73× |
| 100 | 10 | 0.589 | 1.014 | 1.72× |
| 1,000 | 1 | 0.070 | 0.112 | 1.61× |
| 1,000 | 5 | 0.322 | 0.576 | 1.79× |
| 1,000 | 10 | 0.653 | 1.105 | 1.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
