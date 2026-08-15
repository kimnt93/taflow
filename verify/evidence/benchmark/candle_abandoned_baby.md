# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.25M | 0.012 | 83.36M | 0.039 | 2.51× | 3.26× |
| 10,000 | 0.158 | 63.35M | 0.155 | 64.61M | 0.134 | 0.85× | 0.87× |
| 100,000 | 1.666 | 60.01M | 1.618 | 61.80M | 0.987 | 0.59× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.125 | 1.70× |
| 1 | 5 | 0.223 | 0.492 | 2.21× |
| 1 | 10 | 0.382 | 0.983 | 2.57× |
| 10 | 1 | 0.043 | 0.094 | 2.21× |
| 10 | 5 | 0.187 | 0.476 | 2.54× |
| 10 | 10 | 0.439 | 1.005 | 2.29× |
| 100 | 1 | 0.052 | 0.096 | 1.85× |
| 100 | 5 | 0.202 | 0.506 | 2.51× |
| 100 | 10 | 0.432 | 1.040 | 2.41× |
| 1,000 | 1 | 0.060 | 0.103 | 1.72× |
| 1,000 | 5 | 0.219 | 0.497 | 2.27× |
| 1,000 | 10 | 0.432 | 1.060 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
