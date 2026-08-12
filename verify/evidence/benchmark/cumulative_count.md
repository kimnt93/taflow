# CumulativeCount benchmark (`one-based cumulative count` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 315.26M | 0.002 | 435.48M | 0.012 | 3.89× | 5.38× |
| 10,000 | 0.009 | 1.11G | 0.007 | 1.49G | 0.017 | 1.90× | 2.57× |
| 100,000 | 0.068 | 1.47G | 0.060 | 1.67G | 0.057 | 0.84× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.092 | 1.36× |
| 1 | 5 | 0.333 | 0.292 | 0.88× |
| 1 | 10 | 0.468 | 0.578 | 1.23× |
| 10 | 1 | 0.047 | 0.058 | 1.24× |
| 10 | 5 | 0.217 | 0.267 | 1.23× |
| 10 | 10 | 0.458 | 0.594 | 1.30× |
| 100 | 1 | 0.058 | 0.058 | 1.00× |
| 100 | 5 | 0.227 | 0.276 | 1.22× |
| 100 | 10 | 0.442 | 0.566 | 1.28× |
| 1,000 | 1 | 0.048 | 0.061 | 1.28× |
| 1,000 | 5 | 0.219 | 0.274 | 1.25× |
| 1,000 | 10 | 0.454 | 0.598 | 1.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
