# FibonacciProjection benchmark (`FibProjection` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.77M | 0.014 | 71.95M | 0.538 | 33.26× | 38.74× |
| 10,000 | 0.138 | 72.38M | 0.136 | 73.38M | 4.235 | 30.65× | 31.07× |
| 100,000 | 1.371 | 72.95M | 1.297 | 77.13M | 47.205 | 34.44× | 36.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.242 | 3.32× |
| 1 | 5 | 0.263 | 0.885 | 3.36× |
| 1 | 10 | 0.419 | 1.848 | 4.41× |
| 10 | 1 | 0.045 | 0.171 | 3.81× |
| 10 | 5 | 0.214 | 0.870 | 4.06× |
| 10 | 10 | 0.402 | 1.941 | 4.83× |
| 100 | 1 | 0.052 | 0.251 | 4.78× |
| 100 | 5 | 0.230 | 1.055 | 4.58× |
| 100 | 10 | 0.425 | 2.364 | 5.56× |
| 1,000 | 1 | 0.060 | 0.806 | 13.35× |
| 1,000 | 5 | 0.209 | 3.481 | 16.68× |
| 1,000 | 10 | 0.468 | 6.807 | 14.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
