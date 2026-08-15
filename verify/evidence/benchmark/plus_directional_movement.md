# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.92M | 0.006 | 170.07M | 0.039 | 5.23× | 6.65× |
| 10,000 | 0.057 | 176.26M | 0.054 | 186.48M | 0.083 | 1.47× | 1.55× |
| 100,000 | 0.518 | 193.10M | 0.497 | 201.16M | 0.529 | 1.02× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.127 | 1.38× |
| 1 | 5 | 0.282 | 0.527 | 1.87× |
| 1 | 10 | 0.412 | 0.973 | 2.37× |
| 10 | 1 | 0.051 | 0.100 | 1.96× |
| 10 | 5 | 0.182 | 0.439 | 2.41× |
| 10 | 10 | 0.404 | 0.992 | 2.45× |
| 100 | 1 | 0.045 | 0.088 | 1.96× |
| 100 | 5 | 0.184 | 0.440 | 2.39× |
| 100 | 10 | 0.421 | 1.005 | 2.39× |
| 1,000 | 1 | 0.051 | 0.095 | 1.86× |
| 1,000 | 5 | 0.224 | 0.505 | 2.25× |
| 1,000 | 10 | 0.396 | 1.004 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
