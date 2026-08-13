# CumulativeMinimum benchmark (`numpy.minimum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.18M | 0.018 | 55.17M | 0.017 | 0.75× | 0.93× |
| 10,000 | 0.131 | 76.59M | 0.119 | 83.94M | 0.038 | 0.29× | 0.32× |
| 100,000 | 1.126 | 88.81M | 1.159 | 86.25M | 0.283 | 0.25× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.234 | 0.089 | 0.38× |
| 1 | 5 | 0.492 | 0.279 | 0.57× |
| 1 | 10 | 0.531 | 0.556 | 1.05× |
| 10 | 1 | 0.065 | 0.066 | 1.01× |
| 10 | 5 | 0.281 | 0.271 | 0.97× |
| 10 | 10 | 0.578 | 0.568 | 0.98× |
| 100 | 1 | 0.071 | 0.054 | 0.75× |
| 100 | 5 | 0.278 | 0.265 | 0.95× |
| 100 | 10 | 0.553 | 0.556 | 1.01× |
| 1,000 | 1 | 0.075 | 0.060 | 0.79× |
| 1,000 | 5 | 0.282 | 0.301 | 1.07× |
| 1,000 | 10 | 0.625 | 0.679 | 1.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
