# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.89M | 0.019 | 52.86M | 0.055 | 2.61× | 2.89× |
| 10,000 | 0.182 | 54.83M | 0.180 | 55.41M | 0.198 | 1.08× | 1.10× |
| 100,000 | 2.616 | 38.23M | 1.739 | 57.49M | 2.219 | 0.85× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.151 | 2.11× |
| 1 | 5 | 0.341 | 0.572 | 1.68× |
| 1 | 10 | 0.474 | 1.110 | 2.34× |
| 10 | 1 | 0.043 | 0.101 | 2.37× |
| 10 | 5 | 0.199 | 0.500 | 2.51× |
| 10 | 10 | 0.431 | 1.082 | 2.51× |
| 100 | 1 | 0.046 | 0.118 | 2.54× |
| 100 | 5 | 0.224 | 0.532 | 2.37× |
| 100 | 10 | 0.464 | 1.180 | 2.54× |
| 1,000 | 1 | 0.072 | 0.124 | 1.72× |
| 1,000 | 5 | 0.247 | 0.614 | 2.48× |
| 1,000 | 10 | 0.465 | 1.418 | 3.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
