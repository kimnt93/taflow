# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.65M | 0.005 | 184.21M | 0.041 | 5.96× | 7.54× |
| 10,000 | 0.054 | 184.12M | 0.046 | 219.43M | 0.087 | 1.61× | 1.91× |
| 100,000 | 0.481 | 207.86M | 0.443 | 225.98M | 0.576 | 1.20× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.129 | 1.20× |
| 1 | 5 | 0.255 | 0.474 | 1.86× |
| 1 | 10 | 0.393 | 0.989 | 2.52× |
| 10 | 1 | 0.046 | 0.109 | 2.39× |
| 10 | 5 | 0.203 | 0.473 | 2.33× |
| 10 | 10 | 0.373 | 0.957 | 2.57× |
| 100 | 1 | 0.041 | 0.090 | 2.18× |
| 100 | 5 | 0.182 | 0.497 | 2.73× |
| 100 | 10 | 0.403 | 0.974 | 2.41× |
| 1,000 | 1 | 0.045 | 0.102 | 2.25× |
| 1,000 | 5 | 0.196 | 0.486 | 2.48× |
| 1,000 | 10 | 0.426 | 1.040 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
