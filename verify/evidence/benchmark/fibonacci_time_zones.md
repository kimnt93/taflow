# FibonacciTimeZones benchmark (`FibTimeZones` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 60.62M | 0.015 | 68.80M | 0.497 | 30.10× | 34.16× |
| 10,000 | 0.142 | 70.52M | 0.140 | 71.43M | 3.505 | 24.72× | 25.04× |
| 100,000 | 1.540 | 64.92M | 1.453 | 68.83M | 39.532 | 25.67× | 27.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.227 | 2.24× |
| 1 | 5 | 0.247 | 0.855 | 3.46× |
| 1 | 10 | 0.485 | 1.928 | 3.97× |
| 10 | 1 | 0.055 | 0.176 | 3.22× |
| 10 | 5 | 0.241 | 0.869 | 3.61× |
| 10 | 10 | 0.529 | 2.051 | 3.88× |
| 100 | 1 | 0.057 | 0.208 | 3.64× |
| 100 | 5 | 0.256 | 1.022 | 3.99× |
| 100 | 10 | 0.548 | 2.395 | 4.37× |
| 1,000 | 1 | 0.070 | 0.621 | 8.91× |
| 1,000 | 5 | 0.271 | 3.135 | 11.56× |
| 1,000 | 10 | 0.527 | 6.376 | 12.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
