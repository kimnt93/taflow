# CumulativeMaximum benchmark (`numpy.maximum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.66M | 0.018 | 55.24M | 0.015 | 0.64× | 0.81× |
| 10,000 | 0.131 | 76.22M | 0.144 | 69.39M | 0.039 | 0.30× | 0.27× |
| 100,000 | 1.194 | 83.77M | 1.177 | 84.93M | 0.270 | 0.23× | 0.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.182 | 0.090 | 0.50× |
| 1 | 5 | 0.357 | 0.293 | 0.82× |
| 1 | 10 | 0.569 | 0.588 | 1.03× |
| 10 | 1 | 0.062 | 0.056 | 0.90× |
| 10 | 5 | 0.275 | 0.288 | 1.05× |
| 10 | 10 | 0.571 | 0.593 | 1.04× |
| 100 | 1 | 0.072 | 0.061 | 0.85× |
| 100 | 5 | 0.287 | 0.277 | 0.96× |
| 100 | 10 | 0.614 | 0.607 | 0.99× |
| 1,000 | 1 | 0.078 | 0.070 | 0.90× |
| 1,000 | 5 | 0.292 | 0.304 | 1.04× |
| 1,000 | 10 | 0.627 | 0.740 | 1.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
