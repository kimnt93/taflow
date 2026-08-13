# CrabPattern benchmark (`Crab` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.98M | 0.051 | 19.77M | 0.223 | 3.79× | 4.41× |
| 10,000 | 0.420 | 23.80M | 0.400 | 24.97M | 1.362 | 3.24× | 3.40× |
| 100,000 | 3.923 | 25.49M | 3.891 | 25.70M | 12.754 | 3.25× | 3.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.210 | 1.87× |
| 1 | 5 | 0.385 | 0.874 | 2.27× |
| 1 | 10 | 0.675 | 1.673 | 2.48× |
| 10 | 1 | 0.078 | 0.178 | 2.29× |
| 10 | 5 | 0.316 | 1.137 | 3.60× |
| 10 | 10 | 0.652 | 1.649 | 2.53× |
| 100 | 1 | 0.074 | 0.177 | 2.38× |
| 100 | 5 | 0.329 | 1.165 | 3.54× |
| 100 | 10 | 0.675 | 1.772 | 2.62× |
| 1,000 | 1 | 0.116 | 0.307 | 2.65× |
| 1,000 | 5 | 0.319 | 1.730 | 5.42× |
| 1,000 | 10 | 0.649 | 2.969 | 4.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
