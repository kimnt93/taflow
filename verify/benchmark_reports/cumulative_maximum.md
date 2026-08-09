# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.04M | 0.006 | 172.65M | 0.053 | 7.77× | 9.12× |
| 10,000 | 0.042 | 236.97M | 0.040 | 250.89M | 0.089 | 2.12× | 2.24× |
| 100,000 | 0.406 | 246.01M | 0.360 | 277.94M | 0.470 | 1.16× | 1.31× |
| 1,000,000 | 4.123 | 242.56M | 4.145 | 241.25M | 4.671 | 1.13× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.250 | 1.72× |
| 1 | 5 | 0.337 | 0.684 | 2.03× |
| 1 | 10 | 0.546 | 1.375 | 2.52× |
| 10 | 1 | 0.054 | 0.149 | 2.78× |
| 10 | 5 | 0.274 | 0.677 | 2.48× |
| 10 | 10 | 0.498 | 1.273 | 2.56× |
| 100 | 1 | 0.053 | 0.157 | 2.94× |
| 100 | 5 | 0.272 | 0.631 | 2.32× |
| 100 | 10 | 0.500 | 1.269 | 2.54× |
| 1,000 | 1 | 0.055 | 0.165 | 2.98× |
| 1,000 | 5 | 0.246 | 0.590 | 2.40× |
| 1,000 | 10 | 0.511 | 1.226 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
