# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.64M | 0.029 | 35.07M | 0.032 | 0.96× | 1.14× |
| 10,000 | 0.236 | 42.46M | 0.246 | 40.68M | 0.079 | 0.34× | 0.32× |
| 100,000 | 2.105 | 47.51M | 2.195 | 45.56M | 0.556 | 0.26× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.115 | 1.27× |
| 1 | 5 | 0.406 | 0.432 | 1.06× |
| 1 | 10 | 0.556 | 0.889 | 1.60× |
| 10 | 1 | 0.065 | 0.087 | 1.33× |
| 10 | 5 | 0.292 | 0.433 | 1.48× |
| 10 | 10 | 0.586 | 0.907 | 1.55× |
| 100 | 1 | 0.071 | 0.088 | 1.24× |
| 100 | 5 | 0.295 | 0.410 | 1.39× |
| 100 | 10 | 0.653 | 0.907 | 1.39× |
| 1,000 | 1 | 0.089 | 0.094 | 1.05× |
| 1,000 | 5 | 0.293 | 0.456 | 1.56× |
| 1,000 | 10 | 0.585 | 0.925 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
