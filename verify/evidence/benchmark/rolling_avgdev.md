# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.13M | 0.021 | 46.95M | 0.051 | 2.52× | 2.41× |
| 10,000 | 0.211 | 47.42M | 0.181 | 55.30M | 0.281 | 1.33× | 1.55× |
| 100,000 | 2.464 | 40.58M | 2.886 | 34.65M | 4.557 | 1.85× | 1.58× |
| 1,000,000 | 32.843 | 30.45M | 34.676 | 28.84M | 41.623 | 1.27× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.192 | 0.271 | 1.41× |
| 1 | 5 | 1.275 | 1.945 | 1.53× |
| 1 | 10 | 1.159 | 3.349 | 2.89× |
| 10 | 1 | 0.406 | 0.255 | 0.63× |
| 10 | 5 | 0.805 | 1.389 | 1.73× |
| 10 | 10 | 4.491 | 6.030 | 1.34× |
| 100 | 1 | 1.468 | 0.243 | 0.17× |
| 100 | 5 | 1.612 | 1.895 | 1.18× |
| 100 | 10 | 2.246 | 3.746 | 1.67× |
| 1,000 | 1 | 2.794 | 3.169 | 1.13× |
| 1,000 | 5 | 0.888 | 2.883 | 3.25× |
| 1,000 | 10 | 6.119 | 4.923 | 0.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
