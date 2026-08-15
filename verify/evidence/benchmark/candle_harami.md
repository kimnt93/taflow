# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.48M | 0.003 | 360.27M | 0.037 | 6.05× | 13.17× |
| 10,000 | 0.060 | 167.13M | 0.054 | 186.46M | 0.140 | 2.34× | 2.61× |
| 100,000 | 0.663 | 150.80M | 0.669 | 149.50M | 1.184 | 1.78× | 1.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.107 | 1.96× |
| 1 | 5 | 0.222 | 0.589 | 2.66× |
| 1 | 10 | 0.461 | 1.023 | 2.22× |
| 10 | 1 | 0.042 | 0.092 | 2.19× |
| 10 | 5 | 0.186 | 0.438 | 2.35× |
| 10 | 10 | 0.436 | 1.050 | 2.41× |
| 100 | 1 | 0.048 | 0.092 | 1.91× |
| 100 | 5 | 0.215 | 0.455 | 2.12× |
| 100 | 10 | 0.425 | 1.115 | 2.62× |
| 1,000 | 1 | 0.059 | 0.135 | 2.30× |
| 1,000 | 5 | 0.221 | 0.512 | 2.32× |
| 1,000 | 10 | 0.479 | 1.043 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
