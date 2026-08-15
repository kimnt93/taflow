# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.11M | 0.010 | 104.12M | 0.041 | 3.11× | 4.25× |
| 10,000 | 0.146 | 68.46M | 0.138 | 72.51M | 0.168 | 1.15× | 1.22× |
| 100,000 | 1.444 | 69.27M | 1.435 | 69.68M | 1.372 | 0.95× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.107 | 1.51× |
| 1 | 5 | 0.359 | 0.527 | 1.47× |
| 1 | 10 | 0.410 | 0.950 | 2.32× |
| 10 | 1 | 0.045 | 0.085 | 1.89× |
| 10 | 5 | 0.190 | 0.454 | 2.39× |
| 10 | 10 | 0.388 | 0.926 | 2.39× |
| 100 | 1 | 0.047 | 0.104 | 2.19× |
| 100 | 5 | 0.185 | 0.442 | 2.39× |
| 100 | 10 | 0.380 | 0.899 | 2.37× |
| 1,000 | 1 | 0.057 | 0.101 | 1.76× |
| 1,000 | 5 | 0.191 | 0.520 | 2.72× |
| 1,000 | 10 | 0.441 | 1.036 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
