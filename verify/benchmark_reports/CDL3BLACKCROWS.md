# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.06M | 0.007 | 141.07M | 0.031 | 3.37× | 4.36× |
| 10,000 | 0.057 | 175.01M | 0.057 | 176.20M | 0.084 | 1.48× | 1.49× |
| 100,000 | 0.744 | 134.34M | 0.716 | 139.62M | 0.639 | 0.86× | 0.89× |
| 1,000,000 | 7.568 | 132.13M | 7.498 | 133.36M | 6.293 | 0.83× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.125 | 1.07× |
| 1 | 5 | 0.312 | 0.493 | 1.58× |
| 1 | 10 | 0.543 | 0.967 | 1.78× |
| 10 | 1 | 0.062 | 0.105 | 1.69× |
| 10 | 5 | 0.260 | 0.459 | 1.77× |
| 10 | 10 | 0.571 | 0.973 | 1.70× |
| 100 | 1 | 0.058 | 0.093 | 1.62× |
| 100 | 5 | 0.267 | 0.482 | 1.81× |
| 100 | 10 | 0.566 | 1.020 | 1.80× |
| 1,000 | 1 | 0.070 | 0.102 | 1.46× |
| 1,000 | 5 | 0.278 | 0.517 | 1.86× |
| 1,000 | 10 | 0.601 | 1.077 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
