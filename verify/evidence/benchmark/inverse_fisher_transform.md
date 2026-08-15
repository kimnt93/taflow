# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.61M | 0.004 | 228.59M | 0.164 | 32.21× | 37.45× |
| 10,000 | 0.034 | 291.09M | 0.032 | 314.89M | 0.487 | 14.17× | 15.32× |
| 100,000 | 0.314 | 318.00M | 0.293 | 341.29M | 3.374 | 10.73× | 11.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.253 | 3.16× |
| 1 | 5 | 0.333 | 1.330 | 3.99× |
| 1 | 10 | 0.394 | 2.217 | 5.63× |
| 10 | 1 | 0.052 | 0.219 | 4.21× |
| 10 | 5 | 0.215 | 1.241 | 5.78× |
| 10 | 10 | 0.389 | 2.273 | 5.84× |
| 100 | 1 | 0.049 | 0.210 | 4.26× |
| 100 | 5 | 0.187 | 1.222 | 6.53× |
| 100 | 10 | 0.387 | 2.288 | 5.91× |
| 1,000 | 1 | 0.054 | 0.260 | 4.82× |
| 1,000 | 5 | 0.191 | 1.355 | 7.10× |
| 1,000 | 10 | 0.397 | 2.597 | 6.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
