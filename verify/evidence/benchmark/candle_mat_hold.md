# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.34M | 0.022 | 45.59M | 0.044 | 1.51× | 2.01× |
| 10,000 | 0.234 | 42.77M | 0.220 | 45.51M | 0.145 | 0.62× | 0.66× |
| 100,000 | 2.068 | 48.35M | 2.205 | 45.35M | 1.041 | 0.50× | 0.47× |
| 1,000,000 | 23.322 | 42.88M | 24.279 | 41.19M | 9.784 | 0.42× | 0.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.152 | 1.67× |
| 1 | 5 | 0.324 | 0.585 | 1.80× |
| 1 | 10 | 0.625 | 1.465 | 2.34× |
| 10 | 1 | 0.066 | 0.105 | 1.60× |
| 10 | 5 | 0.341 | 0.558 | 1.64× |
| 10 | 10 | 0.683 | 1.137 | 1.66× |
| 100 | 1 | 0.067 | 0.109 | 1.63× |
| 100 | 5 | 0.327 | 0.547 | 1.67× |
| 100 | 10 | 0.703 | 1.271 | 1.81× |
| 1,000 | 1 | 0.100 | 0.129 | 1.29× |
| 1,000 | 5 | 0.372 | 0.650 | 1.75× |
| 1,000 | 10 | 0.772 | 1.333 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
