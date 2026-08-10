# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.43M | 0.017 | 60.04M | 0.037 | 1.84× | 2.19× |
| 10,000 | 0.139 | 71.83M | 0.134 | 74.60M | 0.131 | 0.94× | 0.98× |
| 100,000 | 1.379 | 72.50M | 1.398 | 71.55M | 1.114 | 0.81× | 0.80× |
| 1,000,000 | 15.042 | 66.48M | 13.558 | 73.76M | 9.990 | 0.66× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.121 | 1.41× |
| 1 | 5 | 0.353 | 0.511 | 1.45× |
| 1 | 10 | 0.554 | 0.958 | 1.73× |
| 10 | 1 | 0.055 | 0.093 | 1.69× |
| 10 | 5 | 0.250 | 0.444 | 1.77× |
| 10 | 10 | 0.566 | 0.972 | 1.72× |
| 100 | 1 | 0.059 | 0.092 | 1.57× |
| 100 | 5 | 0.261 | 0.438 | 1.68× |
| 100 | 10 | 0.530 | 0.982 | 1.85× |
| 1,000 | 1 | 0.078 | 0.109 | 1.40× |
| 1,000 | 5 | 0.285 | 0.512 | 1.80× |
| 1,000 | 10 | 0.576 | 1.078 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
