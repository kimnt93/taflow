# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.67M | 0.012 | 81.90M | 0.078 | 4.34× | 6.39× |
| 10,000 | 0.148 | 67.42M | 0.101 | 98.65M | 0.136 | 0.92× | 1.34× |
| 100,000 | 0.992 | 100.82M | 0.960 | 104.16M | 1.109 | 1.12× | 1.16× |
| 1,000,000 | 9.832 | 101.71M | 8.825 | 113.32M | 8.391 | 0.85× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.059 | 0.114 | 1.92× |
| 1 | 5 | 0.285 | 0.505 | 1.77× |
| 1 | 10 | 0.510 | 1.318 | 2.59× |
| 10 | 1 | 0.055 | 0.110 | 2.01× |
| 10 | 5 | 0.246 | 0.494 | 2.01× |
| 10 | 10 | 0.556 | 1.143 | 2.06× |
| 100 | 1 | 0.063 | 0.124 | 1.96× |
| 100 | 5 | 0.369 | 0.515 | 1.40× |
| 100 | 10 | 0.545 | 1.058 | 1.94× |
| 1,000 | 1 | 0.061 | 0.111 | 1.83× |
| 1,000 | 5 | 0.324 | 0.667 | 2.06× |
| 1,000 | 10 | 0.583 | 1.101 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
