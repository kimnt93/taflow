# EaseOfMovement benchmark (`EaseOfMovement` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.77M | 0.012 | 86.77M | 0.279 | 15.26× | 24.18× |
| 10,000 | 0.074 | 134.32M | 0.075 | 134.20M | 1.275 | 17.13× | 17.11× |
| 100,000 | 0.674 | 148.35M | 0.696 | 143.66M | 10.189 | 15.11× | 14.64× |
| 1,000,000 | 7.068 | 141.48M | 6.618 | 151.10M | 121.707 | 17.22× | 18.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.309 | 3.74× |
| 1 | 5 | 0.440 | 1.838 | 4.18× |
| 1 | 10 | 0.770 | 3.529 | 4.58× |
| 10 | 1 | 0.066 | 0.245 | 3.73× |
| 10 | 5 | 0.274 | 1.696 | 6.19× |
| 10 | 10 | 0.581 | 2.799 | 4.82× |
| 100 | 1 | 0.059 | 0.273 | 4.60× |
| 100 | 5 | 0.248 | 1.588 | 6.42× |
| 100 | 10 | 0.511 | 3.011 | 5.89× |
| 1,000 | 1 | 0.065 | 0.380 | 5.85× |
| 1,000 | 5 | 0.254 | 2.091 | 8.23× |
| 1,000 | 10 | 0.533 | 3.807 | 7.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
