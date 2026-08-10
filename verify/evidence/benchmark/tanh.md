# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.05M | 0.006 | 175.58M | 0.030 | 4.19× | 5.34× |
| 10,000 | 0.033 | 304.25M | 0.030 | 336.66M | 0.057 | 1.73× | 1.91× |
| 100,000 | 0.309 | 323.88M | 0.290 | 345.07M | 0.340 | 1.10× | 1.17× |
| 1,000,000 | 3.486 | 286.86M | 2.987 | 334.83M | 3.161 | 0.91× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.110 | 1.28× |
| 1 | 5 | 0.257 | 0.436 | 1.69× |
| 1 | 10 | 0.532 | 0.978 | 1.84× |
| 10 | 1 | 0.054 | 0.086 | 1.60× |
| 10 | 5 | 0.251 | 0.410 | 1.64× |
| 10 | 10 | 0.473 | 1.049 | 2.22× |
| 100 | 1 | 0.059 | 0.099 | 1.70× |
| 100 | 5 | 0.246 | 0.428 | 1.74× |
| 100 | 10 | 0.481 | 0.943 | 1.96× |
| 1,000 | 1 | 0.063 | 0.109 | 1.73× |
| 1,000 | 5 | 0.279 | 0.476 | 1.70× |
| 1,000 | 10 | 0.508 | 0.926 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
