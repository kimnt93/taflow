# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.44M | 0.017 | 59.40M | 0.033 | 1.88× | 1.94× |
| 10,000 | 0.143 | 69.84M | 0.151 | 66.26M | 0.118 | 0.82× | 0.78× |
| 100,000 | 1.396 | 71.63M | 1.517 | 65.92M | 1.206 | 0.86× | 0.79× |
| 1,000,000 | 19.135 | 52.26M | 15.882 | 62.97M | 8.965 | 0.47× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.111 | 1.54× |
| 1 | 5 | 0.276 | 0.442 | 1.60× |
| 1 | 10 | 0.529 | 0.909 | 1.72× |
| 10 | 1 | 0.058 | 0.085 | 1.48× |
| 10 | 5 | 0.246 | 0.460 | 1.88× |
| 10 | 10 | 0.554 | 0.928 | 1.67× |
| 100 | 1 | 0.064 | 0.087 | 1.37× |
| 100 | 5 | 0.256 | 0.456 | 1.78× |
| 100 | 10 | 0.555 | 0.922 | 1.66× |
| 1,000 | 1 | 0.070 | 0.098 | 1.41× |
| 1,000 | 5 | 0.260 | 0.472 | 1.81× |
| 1,000 | 10 | 0.592 | 1.023 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
