# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 86.98M | 0.009 | 116.45M | 0.036 | 3.16× | 4.23× |
| 10,000 | 0.064 | 156.96M | 0.079 | 125.98M | 0.084 | 1.31× | 1.05× |
| 100,000 | 0.585 | 170.90M | 0.563 | 177.57M | 0.705 | 1.20× | 1.25× |
| 1,000,000 | 6.345 | 157.60M | 5.644 | 177.18M | 5.466 | 0.86× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.121 | 1.65× |
| 1 | 5 | 0.295 | 0.547 | 1.85× |
| 1 | 10 | 0.514 | 1.026 | 2.00× |
| 10 | 1 | 0.055 | 0.089 | 1.62× |
| 10 | 5 | 0.263 | 0.473 | 1.80× |
| 10 | 10 | 0.580 | 0.962 | 1.66× |
| 100 | 1 | 0.053 | 0.090 | 1.70× |
| 100 | 5 | 0.249 | 0.516 | 2.08× |
| 100 | 10 | 0.569 | 1.018 | 1.79× |
| 1,000 | 1 | 0.055 | 0.090 | 1.64× |
| 1,000 | 5 | 0.248 | 0.495 | 2.00× |
| 1,000 | 10 | 0.543 | 1.066 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
