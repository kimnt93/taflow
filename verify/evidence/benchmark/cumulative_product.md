# CumulativeProduct benchmark (`numpy.cumprod` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.76M | 0.016 | 62.97M | 0.016 | 0.77× | 1.02× |
| 10,000 | 0.106 | 94.12M | 0.094 | 106.75M | 0.034 | 0.32× | 0.36× |
| 100,000 | 0.863 | 115.83M | 0.860 | 116.34M | 0.218 | 0.25× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.089 | 0.93× |
| 1 | 5 | 0.355 | 0.323 | 0.91× |
| 1 | 10 | 0.576 | 0.594 | 1.03× |
| 10 | 1 | 0.064 | 0.060 | 0.93× |
| 10 | 5 | 0.268 | 0.290 | 1.08× |
| 10 | 10 | 0.571 | 0.637 | 1.12× |
| 100 | 1 | 0.064 | 0.068 | 1.05× |
| 100 | 5 | 0.274 | 0.275 | 1.00× |
| 100 | 10 | 0.553 | 0.598 | 1.08× |
| 1,000 | 1 | 0.067 | 0.067 | 1.00× |
| 1,000 | 5 | 0.270 | 0.340 | 1.26× |
| 1,000 | 10 | 0.577 | 0.739 | 1.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
