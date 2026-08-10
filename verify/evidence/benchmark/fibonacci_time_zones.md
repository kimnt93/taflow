# FibonacciTimeZones benchmark (`FibTimeZones` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 64.36M | 0.014 | 69.50M | 0.453 | 29.18× | 31.52× |
| 10,000 | 0.157 | 63.55M | 0.145 | 68.97M | 3.487 | 22.16× | 24.05× |
| 100,000 | 1.435 | 69.70M | 1.377 | 72.62M | 37.533 | 26.16× | 27.26× |
| 1,000,000 | 15.356 | 65.12M | 14.019 | 71.33M | 394.491 | 25.69× | 28.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.221 | 2.99× |
| 1 | 5 | 0.296 | 0.832 | 2.81× |
| 1 | 10 | 0.477 | 1.878 | 3.94× |
| 10 | 1 | 0.054 | 0.183 | 3.37× |
| 10 | 5 | 0.226 | 0.828 | 3.67× |
| 10 | 10 | 0.499 | 1.876 | 3.76× |
| 100 | 1 | 0.059 | 0.203 | 3.42× |
| 100 | 5 | 0.240 | 1.018 | 4.24× |
| 100 | 10 | 0.532 | 2.222 | 4.18× |
| 1,000 | 1 | 0.082 | 0.708 | 8.63× |
| 1,000 | 5 | 0.258 | 3.177 | 12.32× |
| 1,000 | 10 | 0.516 | 11.952 | 23.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
