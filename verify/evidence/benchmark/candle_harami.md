# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.81M | 0.011 | 90.30M | 0.036 | 2.47× | 3.29× |
| 10,000 | 0.135 | 74.13M | 0.127 | 78.59M | 0.149 | 1.10× | 1.17× |
| 100,000 | 1.378 | 72.59M | 1.315 | 76.06M | 1.139 | 0.83× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.142 | 1.14× |
| 1 | 5 | 0.262 | 0.489 | 1.86× |
| 1 | 10 | 0.379 | 0.907 | 2.39× |
| 10 | 1 | 0.042 | 0.087 | 2.07× |
| 10 | 5 | 0.180 | 0.442 | 2.46× |
| 10 | 10 | 0.436 | 0.909 | 2.08× |
| 100 | 1 | 0.042 | 0.092 | 2.17× |
| 100 | 5 | 0.191 | 0.432 | 2.26× |
| 100 | 10 | 0.385 | 0.993 | 2.58× |
| 1,000 | 1 | 0.061 | 0.100 | 1.64× |
| 1,000 | 5 | 0.203 | 0.478 | 2.36× |
| 1,000 | 10 | 0.435 | 1.034 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
