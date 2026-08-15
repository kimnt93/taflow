# FibonacciExtension benchmark (`FibExtension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.42M | 0.014 | 72.42M | 0.584 | 33.52× | 42.27× |
| 10,000 | 0.146 | 68.29M | 0.134 | 74.50M | 4.619 | 31.54× | 34.41× |
| 100,000 | 1.533 | 65.25M | 1.237 | 80.83M | 51.491 | 33.60× | 41.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.215 | 3.16× |
| 1 | 5 | 0.230 | 0.824 | 3.58× |
| 1 | 10 | 0.409 | 1.859 | 4.55× |
| 10 | 1 | 0.042 | 0.171 | 4.05× |
| 10 | 5 | 0.186 | 0.810 | 4.35× |
| 10 | 10 | 0.457 | 1.935 | 4.24× |
| 100 | 1 | 0.053 | 0.212 | 3.99× |
| 100 | 5 | 0.192 | 1.059 | 5.53× |
| 100 | 10 | 0.452 | 2.361 | 5.23× |
| 1,000 | 1 | 0.062 | 0.846 | 13.66× |
| 1,000 | 5 | 0.219 | 3.658 | 16.69× |
| 1,000 | 10 | 0.434 | 7.439 | 17.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
