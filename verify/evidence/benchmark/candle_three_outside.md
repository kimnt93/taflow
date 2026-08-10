# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.68M | 0.008 | 125.76M | 0.030 | 2.57× | 3.73× |
| 10,000 | 0.070 | 143.37M | 0.068 | 147.52M | 0.088 | 1.26× | 1.30× |
| 100,000 | 0.779 | 128.37M | 0.754 | 132.56M | 0.616 | 0.79× | 0.82× |
| 1,000,000 | 8.369 | 119.48M | 7.734 | 129.29M | 5.938 | 0.71× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.106 | 1.29× |
| 1 | 5 | 0.377 | 0.764 | 2.03× |
| 1 | 10 | 0.551 | 0.917 | 1.66× |
| 10 | 1 | 0.053 | 0.091 | 1.71× |
| 10 | 5 | 0.246 | 0.429 | 1.74× |
| 10 | 10 | 0.568 | 0.936 | 1.65× |
| 100 | 1 | 0.066 | 0.097 | 1.48× |
| 100 | 5 | 0.247 | 0.425 | 1.72× |
| 100 | 10 | 0.538 | 0.996 | 1.85× |
| 1,000 | 1 | 0.075 | 0.105 | 1.40× |
| 1,000 | 5 | 0.273 | 0.479 | 1.76× |
| 1,000 | 10 | 0.581 | 1.018 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
