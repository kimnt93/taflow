# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.67M | 0.009 | 110.91M | 0.035 | 3.48× | 3.83× |
| 10,000 | 0.124 | 80.71M | 0.120 | 83.29M | 0.204 | 1.64× | 1.70× |
| 100,000 | 1.346 | 74.30M | 1.343 | 74.45M | 1.798 | 1.34× | 1.34× |
| 1,000,000 | 14.308 | 69.89M | 13.909 | 71.89M | 18.415 | 1.29× | 1.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.152 | 1.22× |
| 1 | 5 | 0.318 | 0.513 | 1.61× |
| 1 | 10 | 0.535 | 0.974 | 1.82× |
| 10 | 1 | 0.051 | 0.094 | 1.84× |
| 10 | 5 | 0.253 | 0.485 | 1.91× |
| 10 | 10 | 0.569 | 1.023 | 1.80× |
| 100 | 1 | 0.063 | 0.103 | 1.63× |
| 100 | 5 | 0.282 | 0.510 | 1.81× |
| 100 | 10 | 0.635 | 1.049 | 1.65× |
| 1,000 | 1 | 0.071 | 0.112 | 1.58× |
| 1,000 | 5 | 0.271 | 0.574 | 2.12× |
| 1,000 | 10 | 0.560 | 1.178 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
