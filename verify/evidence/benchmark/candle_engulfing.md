# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.87M | 0.008 | 127.21M | 0.029 | 2.51× | 3.63× |
| 10,000 | 0.075 | 132.66M | 0.075 | 133.91M | 0.079 | 1.04× | 1.05× |
| 100,000 | 0.734 | 136.21M | 0.744 | 134.41M | 0.560 | 0.76× | 0.75× |
| 1,000,000 | 7.466 | 133.95M | 7.205 | 138.79M | 5.317 | 0.71× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.133 | 1.36× |
| 1 | 5 | 0.402 | 0.517 | 1.29× |
| 1 | 10 | 0.551 | 0.909 | 1.65× |
| 10 | 1 | 0.052 | 0.088 | 1.70× |
| 10 | 5 | 0.239 | 0.415 | 1.73× |
| 10 | 10 | 0.516 | 0.913 | 1.77× |
| 100 | 1 | 0.060 | 0.096 | 1.62× |
| 100 | 5 | 0.258 | 0.423 | 1.64× |
| 100 | 10 | 0.530 | 0.989 | 1.87× |
| 1,000 | 1 | 0.070 | 0.104 | 1.49× |
| 1,000 | 5 | 0.271 | 0.462 | 1.71× |
| 1,000 | 10 | 0.555 | 0.956 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
