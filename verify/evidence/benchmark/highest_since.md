# HighestSince benchmark (`highest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.62M | 0.006 | 156.47M | 0.291 | 38.03× | 45.56× |
| 10,000 | 0.041 | 246.35M | 0.037 | 268.70M | 2.684 | 66.12× | 72.12× |
| 100,000 | 0.357 | 280.42M | 0.383 | 260.87M | 26.645 | 74.72× | 69.51× |
| 1,000,000 | 3.625 | 275.86M | 4.560 | 219.30M | 275.666 | 76.05× | 60.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.087 | 0.88× |
| 1 | 5 | 0.269 | 0.343 | 1.28× |
| 1 | 10 | 0.529 | 0.704 | 1.33× |
| 10 | 1 | 0.049 | 0.073 | 1.47× |
| 10 | 5 | 0.227 | 0.353 | 1.56× |
| 10 | 10 | 0.494 | 0.748 | 1.51× |
| 100 | 1 | 0.053 | 0.094 | 1.79× |
| 100 | 5 | 0.253 | 0.458 | 1.81× |
| 100 | 10 | 0.512 | 1.007 | 1.97× |
| 1,000 | 1 | 0.058 | 0.346 | 5.96× |
| 1,000 | 5 | 0.238 | 1.738 | 7.32× |
| 1,000 | 10 | 0.552 | 3.751 | 6.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
