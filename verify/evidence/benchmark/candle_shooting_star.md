# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.42M | 0.014 | 72.94M | 0.042 | 2.39× | 3.03× |
| 10,000 | 0.143 | 69.71M | 0.137 | 73.11M | 0.163 | 1.14× | 1.19× |
| 100,000 | 1.412 | 70.83M | 2.137 | 46.80M | 1.691 | 1.20× | 0.79× |
| 1,000,000 | 16.260 | 61.50M | 14.504 | 68.95M | 13.093 | 0.81× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.107 | 0.99× |
| 1 | 5 | 0.321 | 0.465 | 1.45× |
| 1 | 10 | 0.684 | 1.130 | 1.65× |
| 10 | 1 | 0.062 | 0.101 | 1.62× |
| 10 | 5 | 0.290 | 0.498 | 1.71× |
| 10 | 10 | 0.597 | 0.961 | 1.61× |
| 100 | 1 | 0.062 | 0.086 | 1.38× |
| 100 | 5 | 0.270 | 0.454 | 1.68× |
| 100 | 10 | 0.572 | 0.914 | 1.60× |
| 1,000 | 1 | 0.074 | 0.111 | 1.50× |
| 1,000 | 5 | 0.293 | 0.537 | 1.83× |
| 1,000 | 10 | 0.600 | 1.106 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
