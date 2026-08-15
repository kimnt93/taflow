# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.47M | 0.004 | 272.28M | 0.038 | 5.54× | 10.45× |
| 10,000 | 0.045 | 222.85M | 0.040 | 249.34M | 0.112 | 2.50× | 2.80× |
| 100,000 | 0.515 | 194.12M | 0.486 | 205.60M | 0.853 | 1.65× | 1.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.124 | 1.83× |
| 1 | 5 | 0.233 | 0.496 | 2.13× |
| 1 | 10 | 0.388 | 0.963 | 2.48× |
| 10 | 1 | 0.044 | 0.100 | 2.29× |
| 10 | 5 | 0.175 | 0.436 | 2.49× |
| 10 | 10 | 0.375 | 0.966 | 2.58× |
| 100 | 1 | 0.047 | 0.106 | 2.26× |
| 100 | 5 | 0.209 | 0.495 | 2.36× |
| 100 | 10 | 0.401 | 0.987 | 2.46× |
| 1,000 | 1 | 0.046 | 0.104 | 2.26× |
| 1,000 | 5 | 0.187 | 0.517 | 2.77× |
| 1,000 | 10 | 0.451 | 1.089 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
