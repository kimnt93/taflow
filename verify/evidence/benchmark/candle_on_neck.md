# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.68M | 0.016 | 63.65M | 0.036 | 1.90× | 2.30× |
| 10,000 | 0.210 | 47.62M | 0.213 | 46.98M | 0.182 | 0.87× | 0.85× |
| 100,000 | 1.932 | 51.76M | 1.512 | 66.15M | 1.018 | 0.53× | 0.67× |
| 1,000,000 | 15.685 | 63.76M | 15.149 | 66.01M | 9.734 | 0.62× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.127 | 1.14× |
| 1 | 5 | 0.388 | 0.472 | 1.22× |
| 1 | 10 | 0.609 | 1.004 | 1.65× |
| 10 | 1 | 0.061 | 0.095 | 1.56× |
| 10 | 5 | 0.273 | 0.440 | 1.61× |
| 10 | 10 | 0.568 | 0.952 | 1.68× |
| 100 | 1 | 0.056 | 0.089 | 1.58× |
| 100 | 5 | 0.259 | 0.425 | 1.64× |
| 100 | 10 | 0.573 | 1.016 | 1.77× |
| 1,000 | 1 | 0.084 | 0.118 | 1.40× |
| 1,000 | 5 | 0.316 | 0.514 | 1.63× |
| 1,000 | 10 | 0.587 | 1.020 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
