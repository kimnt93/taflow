# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.00M | 0.008 | 123.92M | 0.037 | 3.76× | 4.57× |
| 10,000 | 0.093 | 107.93M | 0.090 | 111.71M | 0.131 | 1.42× | 1.47× |
| 100,000 | 1.001 | 99.86M | 0.954 | 104.79M | 1.020 | 1.02× | 1.07× |
| 1,000,000 | 9.812 | 101.92M | 9.777 | 102.28M | 10.374 | 1.06× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.119 | 1.12× |
| 1 | 5 | 0.284 | 0.524 | 1.85× |
| 1 | 10 | 0.496 | 0.920 | 1.86× |
| 10 | 1 | 0.055 | 0.093 | 1.70× |
| 10 | 5 | 0.245 | 0.420 | 1.72× |
| 10 | 10 | 0.500 | 0.924 | 1.85× |
| 100 | 1 | 0.054 | 0.090 | 1.67× |
| 100 | 5 | 0.253 | 0.441 | 1.74× |
| 100 | 10 | 0.540 | 0.933 | 1.73× |
| 1,000 | 1 | 0.067 | 0.102 | 1.53× |
| 1,000 | 5 | 0.249 | 0.495 | 1.99× |
| 1,000 | 10 | 0.533 | 1.053 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
