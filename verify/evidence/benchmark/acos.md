# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.73M | 0.009 | 110.59M | 0.037 | 3.71× | 4.11× |
| 10,000 | 0.074 | 135.24M | 0.073 | 137.38M | 0.098 | 1.33× | 1.35× |
| 100,000 | 1.106 | 90.44M | 0.737 | 135.77M | 0.747 | 0.68× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.116 | 1.36× |
| 1 | 5 | 0.263 | 0.480 | 1.83× |
| 1 | 10 | 0.490 | 0.907 | 1.85× |
| 10 | 1 | 0.047 | 0.088 | 1.87× |
| 10 | 5 | 0.237 | 0.422 | 1.78× |
| 10 | 10 | 0.446 | 0.976 | 2.19× |
| 100 | 1 | 0.065 | 0.085 | 1.31× |
| 100 | 5 | 0.225 | 0.423 | 1.88× |
| 100 | 10 | 0.483 | 0.927 | 1.92× |
| 1,000 | 1 | 0.061 | 0.104 | 1.72× |
| 1,000 | 5 | 0.308 | 0.498 | 1.62× |
| 1,000 | 10 | 0.510 | 0.971 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
