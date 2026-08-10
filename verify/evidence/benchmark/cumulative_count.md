# CumulativeCount benchmark (`one-based cumulative count` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 294.58M | 0.003 | 390.74M | 0.013 | 3.69× | 4.89× |
| 10,000 | 0.009 | 1.12G | 0.006 | 1.56G | 0.017 | 1.95× | 2.70× |
| 100,000 | 0.067 | 1.48G | 0.044 | 2.28G | 0.061 | 0.90× | 1.38× |
| 1,000,000 | 0.796 | 1.26G | 0.410 | 2.44G | 0.497 | 0.62× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.084 | 0.95× |
| 1 | 5 | 0.333 | 0.308 | 0.93× |
| 1 | 10 | 0.453 | 0.582 | 1.28× |
| 10 | 1 | 0.049 | 0.055 | 1.13× |
| 10 | 5 | 0.231 | 0.280 | 1.21× |
| 10 | 10 | 0.457 | 0.620 | 1.36× |
| 100 | 1 | 0.046 | 0.064 | 1.39× |
| 100 | 5 | 0.218 | 0.276 | 1.27× |
| 100 | 10 | 0.455 | 0.612 | 1.34× |
| 1,000 | 1 | 0.050 | 0.069 | 1.39× |
| 1,000 | 5 | 0.247 | 0.290 | 1.17× |
| 1,000 | 10 | 0.485 | 0.618 | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
