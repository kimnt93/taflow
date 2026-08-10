# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.86M | 0.009 | 111.65M | 0.032 | 2.67× | 3.59× |
| 10,000 | 0.069 | 144.85M | 0.062 | 161.11M | 0.109 | 1.58× | 1.76× |
| 100,000 | 0.677 | 147.79M | 0.659 | 151.81M | 0.853 | 1.26× | 1.29× |
| 1,000,000 | 7.170 | 139.48M | 6.903 | 144.87M | 8.429 | 1.18× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.107 | 0.91× |
| 1 | 5 | 0.362 | 0.448 | 1.24× |
| 1 | 10 | 0.527 | 0.875 | 1.66× |
| 10 | 1 | 0.054 | 0.086 | 1.61× |
| 10 | 5 | 0.271 | 0.433 | 1.59× |
| 10 | 10 | 0.520 | 0.913 | 1.76× |
| 100 | 1 | 0.055 | 0.097 | 1.76× |
| 100 | 5 | 0.310 | 0.455 | 1.47× |
| 100 | 10 | 0.538 | 0.908 | 1.69× |
| 1,000 | 1 | 0.065 | 0.095 | 1.47× |
| 1,000 | 5 | 0.266 | 0.468 | 1.76× |
| 1,000 | 10 | 0.572 | 0.988 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
