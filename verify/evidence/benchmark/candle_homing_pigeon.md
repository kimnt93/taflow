# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.90M | 0.003 | 368.69M | 0.031 | 5.23× | 11.48× |
| 10,000 | 0.048 | 210.31M | 0.046 | 218.56M | 0.096 | 2.02× | 2.09× |
| 100,000 | 0.632 | 158.14M | 0.624 | 160.24M | 0.757 | 1.20× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.102 | 1.25× |
| 1 | 5 | 0.229 | 0.433 | 1.89× |
| 1 | 10 | 0.391 | 0.863 | 2.21× |
| 10 | 1 | 0.045 | 0.083 | 1.84× |
| 10 | 5 | 0.194 | 0.446 | 2.30× |
| 10 | 10 | 0.398 | 0.900 | 2.26× |
| 100 | 1 | 0.040 | 0.085 | 2.10× |
| 100 | 5 | 0.180 | 0.422 | 2.34× |
| 100 | 10 | 0.441 | 0.984 | 2.23× |
| 1,000 | 1 | 0.052 | 0.091 | 1.75× |
| 1,000 | 5 | 0.205 | 0.477 | 2.33× |
| 1,000 | 10 | 0.402 | 1.022 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
