# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.62M | 0.080 | 12.51M | 0.041 | 0.47× | 0.51× |
| 10,000 | 0.857 | 11.67M | 0.697 | 14.35M | 0.112 | 0.13× | 0.16× |
| 100,000 | 7.441 | 13.44M | 6.718 | 14.89M | 0.799 | 0.11× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.148 | 1.51× |
| 1 | 5 | 0.462 | 0.519 | 1.12× |
| 1 | 10 | 0.692 | 0.992 | 1.44× |
| 10 | 1 | 0.068 | 0.102 | 1.50× |
| 10 | 5 | 0.309 | 0.484 | 1.57× |
| 10 | 10 | 0.574 | 0.975 | 1.70× |
| 100 | 1 | 0.082 | 0.091 | 1.11× |
| 100 | 5 | 0.286 | 0.461 | 1.61× |
| 100 | 10 | 0.621 | 1.374 | 2.21× |
| 1,000 | 1 | 0.181 | 0.124 | 0.69× |
| 1,000 | 5 | 0.401 | 0.613 | 1.53× |
| 1,000 | 10 | 0.826 | 1.219 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
