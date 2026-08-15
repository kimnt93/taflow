# FibonacciProjection benchmark (`FibProjection` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.91M | 0.013 | 76.43M | 0.511 | 32.65× | 39.05× |
| 10,000 | 0.134 | 74.51M | 0.129 | 77.30M | 4.263 | 31.76× | 32.95× |
| 100,000 | 1.489 | 67.14M | 1.317 | 75.95M | 44.919 | 30.16× | 34.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.203 | 2.73× |
| 1 | 5 | 0.276 | 0.844 | 3.06× |
| 1 | 10 | 0.397 | 1.829 | 4.61× |
| 10 | 1 | 0.048 | 0.168 | 3.52× |
| 10 | 5 | 0.198 | 0.830 | 4.18× |
| 10 | 10 | 0.404 | 1.868 | 4.62× |
| 100 | 1 | 0.059 | 0.227 | 3.87× |
| 100 | 5 | 0.198 | 1.048 | 5.28× |
| 100 | 10 | 0.444 | 2.299 | 5.18× |
| 1,000 | 1 | 0.064 | 0.817 | 12.80× |
| 1,000 | 5 | 0.215 | 3.275 | 15.23× |
| 1,000 | 10 | 0.419 | 6.679 | 15.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
