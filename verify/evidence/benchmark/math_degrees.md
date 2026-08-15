# MathDegrees benchmark (`numpy.degrees` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 550.61M | 0.001 | 1.19G | 0.013 | 7.05× | 15.23× |
| 10,000 | 0.005 | 1.87G | 0.003 | 3.35G | 0.024 | 4.52× | 8.11× |
| 100,000 | 0.048 | 2.09G | 0.026 | 3.79G | 0.130 | 2.72× | 4.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.074 | 0.73× |
| 1 | 5 | 0.236 | 0.309 | 1.31× |
| 1 | 10 | 0.391 | 0.574 | 1.47× |
| 10 | 1 | 0.041 | 0.053 | 1.30× |
| 10 | 5 | 0.183 | 0.265 | 1.45× |
| 10 | 10 | 0.366 | 0.576 | 1.57× |
| 100 | 1 | 0.044 | 0.059 | 1.34× |
| 100 | 5 | 0.197 | 0.347 | 1.76× |
| 100 | 10 | 0.398 | 0.650 | 1.63× |
| 1,000 | 1 | 0.045 | 0.067 | 1.49× |
| 1,000 | 5 | 0.197 | 0.339 | 1.72× |
| 1,000 | 10 | 0.423 | 0.626 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
