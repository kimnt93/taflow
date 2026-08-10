# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.45M | 0.023 | 42.97M | 0.213 | 8.85× | 9.17× |
| 10,000 | 0.237 | 42.22M | 0.229 | 43.73M | 1.039 | 4.39× | 4.55× |
| 100,000 | 2.075 | 48.18M | 2.054 | 48.68M | 8.194 | 3.95× | 3.99× |
| 1,000,000 | 21.014 | 47.59M | 20.399 | 49.02M | 77.133 | 3.67× | 3.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.272 | 3.31× |
| 1 | 5 | 0.277 | 1.329 | 4.80× |
| 1 | 10 | 0.516 | 2.412 | 4.67× |
| 10 | 1 | 0.055 | 0.220 | 3.97× |
| 10 | 5 | 0.256 | 1.314 | 5.14× |
| 10 | 10 | 0.523 | 2.514 | 4.80× |
| 100 | 1 | 0.054 | 0.227 | 4.18× |
| 100 | 5 | 0.273 | 1.350 | 4.94× |
| 100 | 10 | 0.541 | 2.661 | 4.92× |
| 1,000 | 1 | 0.082 | 0.314 | 3.84× |
| 1,000 | 5 | 0.259 | 1.731 | 6.68× |
| 1,000 | 10 | 0.543 | 3.288 | 6.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
