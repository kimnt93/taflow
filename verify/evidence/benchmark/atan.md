# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.65M | 0.007 | 147.80M | 0.035 | 4.61× | 5.17× |
| 10,000 | 0.063 | 158.49M | 0.061 | 164.53M | 0.089 | 1.41× | 1.46× |
| 100,000 | 0.609 | 164.20M | 0.596 | 167.87M | 0.641 | 1.05× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.174 | 2.27× |
| 1 | 5 | 0.215 | 0.424 | 1.97× |
| 1 | 10 | 0.428 | 0.877 | 2.05× |
| 10 | 1 | 0.046 | 0.084 | 1.81× |
| 10 | 5 | 0.218 | 0.473 | 2.17× |
| 10 | 10 | 0.400 | 0.913 | 2.28× |
| 100 | 1 | 0.047 | 0.085 | 1.80× |
| 100 | 5 | 0.182 | 0.451 | 2.47× |
| 100 | 10 | 0.419 | 1.004 | 2.40× |
| 1,000 | 1 | 0.049 | 0.094 | 1.92× |
| 1,000 | 5 | 0.189 | 0.451 | 2.39× |
| 1,000 | 10 | 0.423 | 1.094 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
