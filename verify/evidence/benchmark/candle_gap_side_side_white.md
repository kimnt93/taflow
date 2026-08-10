# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.41M | 0.013 | 74.95M | 0.045 | 2.83× | 3.40× |
| 10,000 | 0.119 | 84.35M | 0.114 | 87.48M | 0.230 | 1.94× | 2.01× |
| 100,000 | 1.249 | 80.09M | 1.239 | 80.73M | 1.912 | 1.53× | 1.54× |
| 1,000,000 | 13.331 | 75.01M | 13.173 | 75.92M | 19.022 | 1.43× | 1.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.110 | 0.66× |
| 1 | 5 | 0.332 | 0.455 | 1.37× |
| 1 | 10 | 0.544 | 0.895 | 1.64× |
| 10 | 1 | 0.055 | 0.090 | 1.64× |
| 10 | 5 | 0.275 | 0.439 | 1.60× |
| 10 | 10 | 0.525 | 0.910 | 1.73× |
| 100 | 1 | 0.057 | 0.088 | 1.54× |
| 100 | 5 | 0.251 | 0.437 | 1.74× |
| 100 | 10 | 0.539 | 0.952 | 1.77× |
| 1,000 | 1 | 0.069 | 0.110 | 1.59× |
| 1,000 | 5 | 0.256 | 0.532 | 2.08× |
| 1,000 | 10 | 0.578 | 1.114 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
