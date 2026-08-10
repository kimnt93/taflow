# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.45M | 0.013 | 78.51M | 0.055 | 3.85× | 4.29× |
| 10,000 | 0.109 | 91.36M | 0.108 | 92.59M | 0.115 | 1.05× | 1.06× |
| 100,000 | 1.007 | 99.33M | 0.910 | 109.93M | 0.708 | 0.70× | 0.78× |
| 1,000,000 | 21.165 | 47.25M | 9.333 | 107.15M | 7.860 | 0.37× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.132 | 1.16× |
| 1 | 5 | 0.414 | 0.598 | 1.44× |
| 1 | 10 | 0.505 | 1.115 | 2.21× |
| 10 | 1 | 0.052 | 0.113 | 2.17× |
| 10 | 5 | 0.238 | 0.561 | 2.36× |
| 10 | 10 | 0.527 | 1.143 | 2.17× |
| 100 | 1 | 0.059 | 0.111 | 1.86× |
| 100 | 5 | 0.243 | 0.558 | 2.30× |
| 100 | 10 | 0.550 | 1.164 | 2.12× |
| 1,000 | 1 | 0.064 | 0.123 | 1.93× |
| 1,000 | 5 | 0.283 | 0.632 | 2.23× |
| 1,000 | 10 | 0.558 | 1.205 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
