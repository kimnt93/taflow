# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.52M | 0.018 | 56.94M | 0.059 | 3.90× | 3.34× |
| 10,000 | 0.109 | 91.84M | 0.096 | 103.88M | 0.119 | 1.09× | 1.24× |
| 100,000 | 1.039 | 96.23M | 0.947 | 105.60M | 0.788 | 0.76× | 0.83× |
| 1,000,000 | 21.198 | 47.17M | 9.978 | 100.22M | 8.760 | 0.41× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.145 | 1.78× |
| 1 | 5 | 0.312 | 0.561 | 1.80× |
| 1 | 10 | 0.524 | 1.187 | 2.27× |
| 10 | 1 | 0.060 | 0.117 | 1.97× |
| 10 | 5 | 0.245 | 0.563 | 2.29× |
| 10 | 10 | 0.525 | 1.125 | 2.14× |
| 100 | 1 | 0.055 | 0.116 | 2.09× |
| 100 | 5 | 0.269 | 0.575 | 2.14× |
| 100 | 10 | 0.558 | 1.172 | 2.10× |
| 1,000 | 1 | 0.071 | 0.117 | 1.63× |
| 1,000 | 5 | 0.268 | 0.632 | 2.36× |
| 1,000 | 10 | 0.606 | 1.247 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
