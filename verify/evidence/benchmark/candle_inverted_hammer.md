# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.79M | 0.017 | 57.32M | 0.051 | 2.36× | 2.95× |
| 10,000 | 0.195 | 51.33M | 0.176 | 56.95M | 0.196 | 1.01× | 1.12× |
| 100,000 | 1.844 | 54.24M | 1.792 | 55.82M | 1.712 | 0.93× | 0.96× |
| 1,000,000 | 18.601 | 53.76M | 19.590 | 51.05M | 16.381 | 0.88× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.126 | 1.37× |
| 1 | 5 | 0.333 | 0.534 | 1.60× |
| 1 | 10 | 0.616 | 1.058 | 1.72× |
| 10 | 1 | 0.064 | 0.113 | 1.77× |
| 10 | 5 | 0.294 | 0.508 | 1.73× |
| 10 | 10 | 0.636 | 1.033 | 1.62× |
| 100 | 1 | 0.068 | 0.088 | 1.28× |
| 100 | 5 | 0.287 | 0.486 | 1.69× |
| 100 | 10 | 0.606 | 1.161 | 1.92× |
| 1,000 | 1 | 0.094 | 0.136 | 1.44× |
| 1,000 | 5 | 0.298 | 0.560 | 1.88× |
| 1,000 | 10 | 0.663 | 1.295 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
