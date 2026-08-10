# RogersSatchell benchmark (`RogersSatchellVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.40M | 0.037 | 26.73M | 0.317 | 7.72× | 8.46× |
| 10,000 | 0.300 | 33.32M | 0.290 | 34.42M | 1.696 | 5.65× | 5.84× |
| 100,000 | 2.840 | 35.21M | 2.813 | 35.55M | 15.650 | 5.51× | 5.56× |
| 1,000,000 | 29.328 | 34.10M | 28.041 | 35.66M | 151.016 | 5.15× | 5.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.277 | 2.27× |
| 1 | 5 | 0.350 | 1.448 | 4.14× |
| 1 | 10 | 0.616 | 2.725 | 4.42× |
| 10 | 1 | 0.061 | 0.249 | 4.06× |
| 10 | 5 | 0.271 | 1.448 | 5.34× |
| 10 | 10 | 0.538 | 2.602 | 4.84× |
| 100 | 1 | 0.059 | 0.249 | 4.20× |
| 100 | 5 | 0.266 | 1.527 | 5.73× |
| 100 | 10 | 0.543 | 2.934 | 5.40× |
| 1,000 | 1 | 0.088 | 0.402 | 4.58× |
| 1,000 | 5 | 0.270 | 2.241 | 8.29× |
| 1,000 | 10 | 0.603 | 4.084 | 6.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
