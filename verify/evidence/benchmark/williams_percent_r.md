# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.23M | 0.028 | 35.46M | 0.036 | 1.18× | 1.26× |
| 10,000 | 0.296 | 33.81M | 0.269 | 37.22M | 0.109 | 0.37× | 0.41× |
| 100,000 | 2.818 | 35.48M | 2.606 | 38.37M | 0.839 | 0.30× | 0.32× |
| 1,000,000 | 30.345 | 32.95M | 26.870 | 37.22M | 8.158 | 0.27× | 0.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.107 | 1.24× |
| 1 | 5 | 0.308 | 0.466 | 1.51× |
| 1 | 10 | 0.479 | 0.945 | 1.97× |
| 10 | 1 | 0.053 | 0.092 | 1.72× |
| 10 | 5 | 0.239 | 0.436 | 1.82× |
| 10 | 10 | 0.498 | 0.930 | 1.87× |
| 100 | 1 | 0.051 | 0.098 | 1.92× |
| 100 | 5 | 0.228 | 0.439 | 1.92× |
| 100 | 10 | 0.501 | 0.935 | 1.87× |
| 1,000 | 1 | 0.082 | 0.121 | 1.47× |
| 1,000 | 5 | 0.242 | 0.486 | 2.01× |
| 1,000 | 10 | 0.539 | 1.056 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
