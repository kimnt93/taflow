# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.09M | 0.025 | 39.29M | 0.100 | 3.91× | 3.93× |
| 10,000 | 0.234 | 42.72M | 0.229 | 43.66M | 0.847 | 3.62× | 3.70× |
| 100,000 | 2.353 | 42.50M | 2.121 | 47.14M | 8.254 | 3.51× | 3.89× |
| 1,000,000 | 22.180 | 45.08M | 22.453 | 44.54M | 81.561 | 3.68× | 3.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.089 | 0.74× |
| 1 | 5 | 0.311 | 0.364 | 1.17× |
| 1 | 10 | 0.506 | 0.758 | 1.50× |
| 10 | 1 | 0.061 | 0.071 | 1.17× |
| 10 | 5 | 0.219 | 0.338 | 1.55× |
| 10 | 10 | 0.628 | 0.708 | 1.13× |
| 100 | 1 | 0.055 | 0.077 | 1.40× |
| 100 | 5 | 0.258 | 0.397 | 1.54× |
| 100 | 10 | 0.490 | 0.858 | 1.75× |
| 1,000 | 1 | 0.075 | 0.157 | 2.08× |
| 1,000 | 5 | 0.271 | 0.845 | 3.12× |
| 1,000 | 10 | 0.572 | 1.641 | 2.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
