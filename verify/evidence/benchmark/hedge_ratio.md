# HedgeRatio benchmark (`rolling OLS hedge ratio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.271 | 3.69M | 0.272 | 3.68M | 0.252 | 0.93× | 0.93× |
| 10,000 | 2.679 | 3.73M | 2.670 | 3.74M | 1.575 | 0.59× | 0.59× |
| 100,000 | 34.614 | 2.89M | 27.437 | 3.64M | 16.481 | 0.48× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.147 | 0.99× |
| 1 | 5 | 0.358 | 0.700 | 1.95× |
| 1 | 10 | 0.647 | 1.272 | 1.96× |
| 10 | 1 | 0.072 | 0.123 | 1.70× |
| 10 | 5 | 0.305 | 0.634 | 2.08× |
| 10 | 10 | 0.613 | 1.278 | 2.09× |
| 100 | 1 | 0.099 | 0.204 | 2.06× |
| 100 | 5 | 0.303 | 1.135 | 3.75× |
| 100 | 10 | 0.668 | 2.287 | 3.42× |
| 1,000 | 1 | 0.372 | 0.356 | 0.96× |
| 1,000 | 5 | 0.689 | 1.322 | 1.92× |
| 1,000 | 10 | 1.002 | 2.759 | 2.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
