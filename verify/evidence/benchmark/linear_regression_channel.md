# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.96M | 0.068 | 14.76M | 0.590 | 8.24× | 8.71× |
| 10,000 | 0.668 | 14.97M | 0.650 | 15.37M | 4.707 | 7.05× | 7.24× |
| 100,000 | 6.705 | 14.91M | 6.632 | 15.08M | 43.202 | 6.44× | 6.51× |
| 1,000,000 | 67.529 | 14.81M | 68.226 | 14.66M | 495.940 | 7.34× | 7.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.284 | 2.14× |
| 1 | 5 | 0.395 | 1.498 | 3.79× |
| 1 | 10 | 0.478 | 2.727 | 5.71× |
| 10 | 1 | 0.058 | 0.257 | 4.43× |
| 10 | 5 | 0.260 | 1.374 | 5.28× |
| 10 | 10 | 0.473 | 2.858 | 6.04× |
| 100 | 1 | 0.059 | 0.292 | 4.96× |
| 100 | 5 | 0.250 | 1.571 | 6.29× |
| 100 | 10 | 0.606 | 3.279 | 5.41× |
| 1,000 | 1 | 0.120 | 0.886 | 7.39× |
| 1,000 | 5 | 0.309 | 3.700 | 11.97× |
| 1,000 | 10 | 0.614 | 7.676 | 12.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
