# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.73M | 0.010 | 96.35M | 0.041 | 3.07× | 3.96× |
| 10,000 | 0.095 | 104.76M | 0.095 | 105.39M | 0.127 | 1.33× | 1.33× |
| 100,000 | 0.959 | 104.26M | 0.921 | 108.56M | 0.964 | 1.01× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.134 | 1.98× |
| 1 | 5 | 0.293 | 0.491 | 1.67× |
| 1 | 10 | 0.397 | 0.993 | 2.50× |
| 10 | 1 | 0.042 | 0.092 | 2.18× |
| 10 | 5 | 0.181 | 0.454 | 2.50× |
| 10 | 10 | 0.418 | 0.963 | 2.30× |
| 100 | 1 | 0.042 | 0.105 | 2.48× |
| 100 | 5 | 0.195 | 0.459 | 2.36× |
| 100 | 10 | 0.401 | 1.013 | 2.52× |
| 1,000 | 1 | 0.059 | 0.109 | 1.86× |
| 1,000 | 5 | 0.192 | 0.508 | 2.65× |
| 1,000 | 10 | 0.415 | 1.076 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
