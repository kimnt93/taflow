# ValueWhen benchmark (`last value when condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.93M | 0.005 | 196.49M | 0.158 | 25.39× | 31.00× |
| 10,000 | 0.026 | 388.21M | 0.023 | 439.65M | 1.474 | 57.24× | 64.82× |
| 100,000 | 0.198 | 506.32M | 0.175 | 570.56M | 14.424 | 73.03× | 82.30× |
| 1,000,000 | 2.232 | 447.94M | 1.818 | 550.02M | 144.418 | 64.69× | 79.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.092 | 0.77× |
| 1 | 5 | 0.316 | 0.329 | 1.04× |
| 1 | 10 | 0.546 | 0.688 | 1.26× |
| 10 | 1 | 0.047 | 0.065 | 1.39× |
| 10 | 5 | 0.237 | 0.326 | 1.37× |
| 10 | 10 | 0.509 | 0.733 | 1.44× |
| 100 | 1 | 0.051 | 0.084 | 1.65× |
| 100 | 5 | 0.232 | 0.378 | 1.63× |
| 100 | 10 | 0.491 | 0.841 | 1.71× |
| 1,000 | 1 | 0.050 | 0.212 | 4.25× |
| 1,000 | 5 | 0.249 | 1.065 | 4.27× |
| 1,000 | 10 | 0.529 | 2.306 | 4.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
