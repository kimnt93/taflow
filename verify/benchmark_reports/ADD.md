# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 197.22M | 0.003 | 294.41M | 0.032 | 6.34× | 9.47× |
| 10,000 | 0.011 | 917.92M | 0.007 | 1.36G | 0.034 | 3.14× | 4.64× |
| 100,000 | 0.071 | 1.41G | 0.043 | 2.32G | 0.075 | 1.06× | 1.75× |
| 1,000,000 | 1.922 | 520.28M | 1.869 | 534.96M | 1.230 | 0.64× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.113 | 0.86× |
| 1 | 5 | 0.282 | 0.488 | 1.73× |
| 1 | 10 | 0.535 | 1.084 | 2.03× |
| 10 | 1 | 0.056 | 0.090 | 1.60× |
| 10 | 5 | 0.264 | 0.467 | 1.77× |
| 10 | 10 | 0.564 | 1.143 | 2.03× |
| 100 | 1 | 0.070 | 0.120 | 1.72× |
| 100 | 5 | 0.264 | 0.489 | 1.85× |
| 100 | 10 | 0.572 | 1.169 | 2.05× |
| 1,000 | 1 | 0.087 | 0.160 | 1.83× |
| 1,000 | 5 | 0.263 | 0.478 | 1.82× |
| 1,000 | 10 | 0.519 | 1.035 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
