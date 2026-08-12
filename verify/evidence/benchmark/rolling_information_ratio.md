# RollingInformationRatio benchmark (`InformationRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.70M | 0.038 | 26.12M | 0.200 | 5.34× | 5.23× |
| 10,000 | 0.318 | 31.42M | 0.313 | 31.91M | 0.784 | 2.46× | 2.50× |
| 100,000 | 3.211 | 31.14M | 3.018 | 33.13M | 7.337 | 2.29× | 2.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.292 | 3.08× |
| 1 | 5 | 0.340 | 1.084 | 3.19× |
| 1 | 10 | 0.466 | 2.351 | 5.05× |
| 10 | 1 | 0.054 | 0.194 | 3.59× |
| 10 | 5 | 0.219 | 0.997 | 4.55× |
| 10 | 10 | 0.488 | 2.446 | 5.01× |
| 100 | 1 | 0.058 | 0.219 | 3.79× |
| 100 | 5 | 0.250 | 1.038 | 4.16× |
| 100 | 10 | 0.529 | 2.356 | 4.46× |
| 1,000 | 1 | 0.086 | 0.278 | 3.24× |
| 1,000 | 5 | 0.246 | 1.375 | 5.60× |
| 1,000 | 10 | 0.542 | 3.061 | 5.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
