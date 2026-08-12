# FibonacciChannel benchmark (`FibChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.40M | 0.018 | 54.97M | 0.541 | 25.11× | 29.75× |
| 10,000 | 0.180 | 55.42M | 0.156 | 64.18M | 4.584 | 25.40× | 29.42× |
| 100,000 | 1.829 | 54.66M | 1.493 | 66.96M | 48.626 | 26.58× | 32.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.214 | 2.13× |
| 1 | 5 | 0.346 | 0.846 | 2.44× |
| 1 | 10 | 0.472 | 1.871 | 3.97× |
| 10 | 1 | 0.049 | 0.172 | 3.52× |
| 10 | 5 | 0.244 | 0.865 | 3.54× |
| 10 | 10 | 0.500 | 1.923 | 3.85× |
| 100 | 1 | 0.056 | 0.221 | 3.95× |
| 100 | 5 | 0.239 | 1.061 | 4.44× |
| 100 | 10 | 0.503 | 2.379 | 4.73× |
| 1,000 | 1 | 0.076 | 0.849 | 11.13× |
| 1,000 | 5 | 0.264 | 3.412 | 12.92× |
| 1,000 | 10 | 0.549 | 6.827 | 12.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
