# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.99M | 0.011 | 93.44M | 0.049 | 3.53× | 4.58× |
| 10,000 | 0.126 | 79.45M | 0.121 | 82.67M | 0.221 | 1.76× | 1.83× |
| 100,000 | 1.314 | 76.12M | 1.282 | 77.99M | 1.927 | 1.47× | 1.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.115 | 1.43× |
| 1 | 5 | 0.214 | 0.459 | 2.14× |
| 1 | 10 | 0.397 | 0.939 | 2.36× |
| 10 | 1 | 0.046 | 0.087 | 1.88× |
| 10 | 5 | 0.177 | 0.438 | 2.47× |
| 10 | 10 | 0.430 | 0.937 | 2.18× |
| 100 | 1 | 0.044 | 0.090 | 2.05× |
| 100 | 5 | 0.187 | 0.436 | 2.33× |
| 100 | 10 | 0.413 | 1.003 | 2.43× |
| 1,000 | 1 | 0.058 | 0.115 | 1.97× |
| 1,000 | 5 | 0.217 | 0.545 | 2.51× |
| 1,000 | 10 | 0.433 | 1.127 | 2.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
