# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.315 | 3.17M | 0.310 | 3.23M | 0.514 | 1.63× | 1.66× |
| 10,000 | 3.259 | 3.07M | 3.194 | 3.13M | 3.732 | 1.15× | 1.17× |
| 100,000 | 33.046 | 3.03M | 32.902 | 3.04M | 36.137 | 1.09× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.277 | 3.18× |
| 1 | 5 | 0.232 | 1.226 | 5.29× |
| 1 | 10 | 0.469 | 2.397 | 5.12× |
| 10 | 1 | 0.055 | 0.220 | 3.98× |
| 10 | 5 | 0.221 | 1.316 | 5.94× |
| 10 | 10 | 0.537 | 2.425 | 4.52× |
| 100 | 1 | 0.081 | 0.246 | 3.05× |
| 100 | 5 | 0.241 | 1.472 | 6.12× |
| 100 | 10 | 0.539 | 2.656 | 4.93× |
| 1,000 | 1 | 0.392 | 0.642 | 1.64× |
| 1,000 | 5 | 0.590 | 3.089 | 5.24× |
| 1,000 | 10 | 1.102 | 6.033 | 5.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
