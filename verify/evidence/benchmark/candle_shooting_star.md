# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.11M | 0.016 | 62.49M | 0.045 | 2.26× | 2.82× |
| 10,000 | 0.173 | 57.66M | 0.161 | 61.94M | 0.181 | 1.04× | 1.12× |
| 100,000 | 1.770 | 56.49M | 1.708 | 58.55M | 1.628 | 0.92× | 0.95× |
| 1,000,000 | 17.502 | 57.14M | 17.110 | 58.44M | 15.796 | 0.90× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.129 | 1.21× |
| 1 | 5 | 0.283 | 0.497 | 1.75× |
| 1 | 10 | 0.523 | 1.088 | 2.08× |
| 10 | 1 | 0.065 | 0.096 | 1.48× |
| 10 | 5 | 0.260 | 0.440 | 1.70× |
| 10 | 10 | 0.531 | 0.939 | 1.77× |
| 100 | 1 | 0.066 | 0.109 | 1.65× |
| 100 | 5 | 0.260 | 0.433 | 1.66× |
| 100 | 10 | 0.543 | 0.945 | 1.74× |
| 1,000 | 1 | 0.073 | 0.111 | 1.53× |
| 1,000 | 5 | 0.295 | 0.545 | 1.84× |
| 1,000 | 10 | 0.575 | 1.068 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
