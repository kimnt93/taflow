# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.48M | 0.016 | 62.41M | 0.036 | 1.72× | 2.27× |
| 10,000 | 0.155 | 64.70M | 0.143 | 69.74M | 0.135 | 0.87× | 0.94× |
| 100,000 | 1.423 | 70.29M | 1.495 | 66.87M | 1.153 | 0.81× | 0.77× |
| 1,000,000 | 15.236 | 65.64M | 14.567 | 68.65M | 11.161 | 0.73× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.135 | 1.60× |
| 1 | 5 | 0.294 | 0.498 | 1.69× |
| 1 | 10 | 0.551 | 1.065 | 1.93× |
| 10 | 1 | 0.085 | 0.121 | 1.42× |
| 10 | 5 | 0.306 | 0.532 | 1.74× |
| 10 | 10 | 0.590 | 1.024 | 1.74× |
| 100 | 1 | 0.061 | 0.094 | 1.55× |
| 100 | 5 | 0.319 | 0.517 | 1.62× |
| 100 | 10 | 0.567 | 0.954 | 1.68× |
| 1,000 | 1 | 0.068 | 0.098 | 1.43× |
| 1,000 | 5 | 0.316 | 0.559 | 1.77× |
| 1,000 | 10 | 0.648 | 1.143 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
