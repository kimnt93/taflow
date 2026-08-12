# MathRadians benchmark (`numpy.radians` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.96M | 0.002 | 437.41M | 0.013 | 2.62× | 5.53× |
| 10,000 | 0.007 | 1.39G | 0.006 | 1.67G | 0.024 | 3.31× | 3.97× |
| 100,000 | 0.055 | 1.81G | 0.032 | 3.11G | 0.130 | 2.35× | 4.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.091 | 0.74× |
| 1 | 5 | 0.381 | 0.311 | 0.82× |
| 1 | 10 | 0.467 | 0.600 | 1.28× |
| 10 | 1 | 0.204 | 0.067 | 0.33× |
| 10 | 5 | 0.244 | 0.302 | 1.24× |
| 10 | 10 | 0.464 | 0.588 | 1.27× |
| 100 | 1 | 0.051 | 0.059 | 1.16× |
| 100 | 5 | 0.224 | 0.271 | 1.21× |
| 100 | 10 | 0.457 | 0.570 | 1.25× |
| 1,000 | 1 | 0.051 | 0.059 | 1.15× |
| 1,000 | 5 | 0.233 | 0.277 | 1.19× |
| 1,000 | 10 | 0.457 | 0.597 | 1.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
