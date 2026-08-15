# Vortex benchmark (`Vortex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.21M | 0.011 | 87.12M | 0.511 | 35.91× | 44.56× |
| 10,000 | 0.118 | 84.90M | 0.100 | 99.61M | 4.020 | 34.13× | 40.04× |
| 100,000 | 0.997 | 100.27M | 0.977 | 102.36M | 42.755 | 42.87× | 43.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.288 | 5.03× |
| 1 | 5 | 0.227 | 1.163 | 5.13× |
| 1 | 10 | 0.467 | 2.254 | 4.82× |
| 10 | 1 | 0.046 | 0.211 | 4.63× |
| 10 | 5 | 0.237 | 1.434 | 6.05× |
| 10 | 10 | 0.430 | 2.398 | 5.58× |
| 100 | 1 | 0.055 | 0.259 | 4.73× |
| 100 | 5 | 0.203 | 1.485 | 7.32× |
| 100 | 10 | 0.461 | 2.741 | 5.94× |
| 1,000 | 1 | 0.058 | 0.802 | 13.86× |
| 1,000 | 5 | 0.231 | 3.352 | 14.51× |
| 1,000 | 10 | 0.423 | 6.939 | 16.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
