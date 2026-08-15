# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.076 | 13.11M | 0.076 | 13.08M | 0.045 | 0.59× | 0.59× |
| 10,000 | 0.824 | 12.13M | 0.967 | 10.34M | 0.147 | 0.18× | 0.15× |
| 100,000 | 9.174 | 10.90M | 9.626 | 10.39M | 1.100 | 0.12× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.136 | 1.47× |
| 1 | 5 | 0.297 | 0.517 | 1.74× |
| 1 | 10 | 0.442 | 0.871 | 1.97× |
| 10 | 1 | 0.068 | 0.096 | 1.40× |
| 10 | 5 | 0.213 | 0.542 | 2.55× |
| 10 | 10 | 0.475 | 0.871 | 1.84× |
| 100 | 1 | 0.049 | 0.123 | 2.48× |
| 100 | 5 | 0.210 | 0.594 | 2.83× |
| 100 | 10 | 0.476 | 1.418 | 2.98× |
| 1,000 | 1 | 0.142 | 0.118 | 0.84× |
| 1,000 | 5 | 0.278 | 0.679 | 2.44× |
| 1,000 | 10 | 0.553 | 1.571 | 2.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
