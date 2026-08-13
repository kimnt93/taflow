# MathCot benchmark (`numpy.tan reciprocal` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.11M | 0.038 | 26.60M | 0.024 | 0.58× | 0.64× |
| 10,000 | 0.320 | 31.25M | 0.339 | 29.50M | 0.134 | 0.42× | 0.40× |
| 100,000 | 3.022 | 33.09M | 3.107 | 32.19M | 1.216 | 0.40× | 0.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.095 | 0.90× |
| 1 | 5 | 0.437 | 0.295 | 0.67× |
| 1 | 10 | 0.559 | 0.592 | 1.06× |
| 10 | 1 | 0.063 | 0.060 | 0.95× |
| 10 | 5 | 0.278 | 0.282 | 1.01× |
| 10 | 10 | 0.541 | 0.584 | 1.08× |
| 100 | 1 | 0.068 | 0.062 | 0.91× |
| 100 | 5 | 0.289 | 0.287 | 0.99× |
| 100 | 10 | 0.598 | 0.614 | 1.03× |
| 1,000 | 1 | 0.095 | 0.081 | 0.84× |
| 1,000 | 5 | 0.296 | 0.333 | 1.12× |
| 1,000 | 10 | 0.603 | 0.791 | 1.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
