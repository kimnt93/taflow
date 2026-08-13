# RollingWinsorize benchmark (`rolling winsorize` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.359 | 2.79M | 0.327 | 3.06M | 0.553 | 1.54× | 1.69× |
| 10,000 | 3.108 | 3.22M | 3.049 | 3.28M | 2.987 | 0.96× | 0.98× |
| 100,000 | 31.012 | 3.22M | 30.725 | 3.25M | 29.031 | 0.94× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.327 | 3.19× |
| 1 | 5 | 0.572 | 1.678 | 2.93× |
| 1 | 10 | 0.640 | 3.231 | 5.05× |
| 10 | 1 | 0.071 | 0.291 | 4.12× |
| 10 | 5 | 0.307 | 1.652 | 5.39× |
| 10 | 10 | 0.610 | 3.207 | 5.26× |
| 100 | 1 | 0.103 | 0.358 | 3.48× |
| 100 | 5 | 0.301 | 1.971 | 6.55× |
| 100 | 10 | 0.614 | 3.843 | 6.26× |
| 1,000 | 1 | 0.396 | 0.658 | 1.66× |
| 1,000 | 5 | 0.638 | 2.408 | 3.78× |
| 1,000 | 10 | 1.130 | 4.899 | 4.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
