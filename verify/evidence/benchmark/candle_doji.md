# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.29M | 0.010 | 96.37M | 0.033 | 2.33× | 3.20× |
| 10,000 | 0.048 | 209.92M | 0.074 | 134.58M | 0.054 | 1.14× | 0.73× |
| 100,000 | 0.365 | 274.00M | 0.349 | 286.54M | 0.265 | 0.72× | 0.76× |
| 1,000,000 | 4.285 | 233.36M | 3.839 | 260.48M | 2.878 | 0.67× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.105 | 0.79× |
| 1 | 5 | 0.369 | 0.483 | 1.31× |
| 1 | 10 | 0.536 | 0.932 | 1.74× |
| 10 | 1 | 0.053 | 0.086 | 1.63× |
| 10 | 5 | 0.267 | 0.468 | 1.75× |
| 10 | 10 | 0.553 | 0.937 | 1.69× |
| 100 | 1 | 0.058 | 0.084 | 1.43× |
| 100 | 5 | 0.251 | 0.459 | 1.83× |
| 100 | 10 | 0.564 | 0.951 | 1.68× |
| 1,000 | 1 | 0.061 | 0.090 | 1.47× |
| 1,000 | 5 | 0.273 | 0.434 | 1.59× |
| 1,000 | 10 | 0.577 | 0.946 | 1.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
