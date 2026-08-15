# FibonacciTimeZones benchmark (`FibTimeZones` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.32M | 0.012 | 81.34M | 0.463 | 32.59× | 37.70× |
| 10,000 | 0.207 | 48.37M | 0.177 | 56.37M | 3.590 | 17.37× | 20.24× |
| 100,000 | 1.468 | 68.13M | 1.397 | 71.60M | 38.255 | 26.06× | 27.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.227 | 2.58× |
| 1 | 5 | 0.265 | 0.892 | 3.37× |
| 1 | 10 | 0.391 | 1.838 | 4.70× |
| 10 | 1 | 0.054 | 0.165 | 3.04× |
| 10 | 5 | 0.176 | 0.900 | 5.10× |
| 10 | 10 | 0.456 | 1.924 | 4.22× |
| 100 | 1 | 0.050 | 0.204 | 4.09× |
| 100 | 5 | 0.200 | 1.079 | 5.38× |
| 100 | 10 | 0.451 | 2.222 | 4.92× |
| 1,000 | 1 | 0.074 | 0.645 | 8.68× |
| 1,000 | 5 | 0.209 | 2.982 | 14.24× |
| 1,000 | 10 | 0.465 | 5.891 | 12.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
