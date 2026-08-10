# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.86M | 0.022 | 46.46M | 0.178 | 7.82× | 8.29× |
| 10,000 | 0.204 | 48.94M | 0.194 | 51.56M | 0.707 | 3.46× | 3.65× |
| 100,000 | 2.005 | 49.87M | 1.878 | 53.24M | 6.566 | 3.27× | 3.50× |
| 1,000,000 | 20.898 | 47.85M | 19.012 | 52.60M | 54.264 | 2.60× | 2.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.296 | 2.72× |
| 1 | 5 | 0.325 | 1.000 | 3.08× |
| 1 | 10 | 0.504 | 2.974 | 5.90× |
| 10 | 1 | 0.076 | 0.321 | 4.21× |
| 10 | 5 | 0.685 | 1.483 | 2.17× |
| 10 | 10 | 0.590 | 2.588 | 4.38× |
| 100 | 1 | 0.079 | 0.224 | 2.83× |
| 100 | 5 | 0.334 | 1.363 | 4.08× |
| 100 | 10 | 0.663 | 2.662 | 4.01× |
| 1,000 | 1 | 0.087 | 0.307 | 3.54× |
| 1,000 | 5 | 0.312 | 1.536 | 4.92× |
| 1,000 | 10 | 0.657 | 3.442 | 5.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
