# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.54M | 0.004 | 272.42M | 0.037 | 5.17× | 10.03× |
| 10,000 | 0.062 | 160.54M | 0.050 | 198.94M | 0.087 | 1.39× | 1.72× |
| 100,000 | 0.818 | 122.28M | 0.679 | 147.31M | 0.607 | 0.74× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.139 | 1.47× |
| 1 | 5 | 0.359 | 0.463 | 1.29× |
| 1 | 10 | 0.416 | 0.902 | 2.17× |
| 10 | 1 | 0.043 | 0.088 | 2.05× |
| 10 | 5 | 0.189 | 0.446 | 2.37× |
| 10 | 10 | 0.399 | 0.905 | 2.27× |
| 100 | 1 | 0.048 | 0.089 | 1.85× |
| 100 | 5 | 0.194 | 0.427 | 2.20× |
| 100 | 10 | 0.416 | 0.937 | 2.25× |
| 1,000 | 1 | 0.051 | 0.097 | 1.92× |
| 1,000 | 5 | 0.192 | 0.461 | 2.40× |
| 1,000 | 10 | 0.407 | 0.966 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
