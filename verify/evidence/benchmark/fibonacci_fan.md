# FibonacciFan benchmark (`FibFan` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.22M | 0.014 | 69.59M | 0.488 | 30.35× | 33.94× |
| 10,000 | 0.145 | 68.95M | 0.145 | 68.75M | 4.168 | 28.74× | 28.66× |
| 100,000 | 1.431 | 69.87M | 1.389 | 71.98M | 42.735 | 29.86× | 30.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.200 | 2.94× |
| 1 | 5 | 0.299 | 0.879 | 2.94× |
| 1 | 10 | 0.401 | 1.909 | 4.76× |
| 10 | 1 | 0.051 | 0.181 | 3.56× |
| 10 | 5 | 0.190 | 0.839 | 4.43× |
| 10 | 10 | 0.412 | 2.105 | 5.11× |
| 100 | 1 | 0.057 | 0.219 | 3.86× |
| 100 | 5 | 0.214 | 1.080 | 5.04× |
| 100 | 10 | 0.458 | 2.401 | 5.24× |
| 1,000 | 1 | 0.075 | 0.763 | 10.16× |
| 1,000 | 5 | 0.233 | 3.396 | 14.58× |
| 1,000 | 10 | 0.461 | 6.492 | 14.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
