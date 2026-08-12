# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.39M | 0.025 | 40.13M | 0.035 | 1.36× | 1.42× |
| 10,000 | 0.275 | 36.37M | 0.272 | 36.73M | 0.113 | 0.41× | 0.42× |
| 100,000 | 2.636 | 37.93M | 2.633 | 37.97M | 0.812 | 0.31× | 0.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.130 | 1.00× |
| 1 | 5 | 0.271 | 0.464 | 1.71× |
| 1 | 10 | 0.496 | 0.897 | 1.81× |
| 10 | 1 | 0.049 | 0.087 | 1.78× |
| 10 | 5 | 0.224 | 0.427 | 1.91× |
| 10 | 10 | 0.476 | 0.951 | 2.00× |
| 100 | 1 | 0.054 | 0.091 | 1.68× |
| 100 | 5 | 0.248 | 0.464 | 1.87× |
| 100 | 10 | 0.532 | 0.961 | 1.81× |
| 1,000 | 1 | 0.087 | 0.101 | 1.17× |
| 1,000 | 5 | 0.268 | 0.502 | 1.87× |
| 1,000 | 10 | 0.557 | 1.013 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
