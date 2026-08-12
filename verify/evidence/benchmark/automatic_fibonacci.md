# AutomaticFibonacci benchmark (`AutoFib` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.12M | 0.024 | 40.87M | 0.700 | 23.19× | 28.61× |
| 10,000 | 0.242 | 41.28M | 0.215 | 46.42M | 6.007 | 24.80× | 27.89× |
| 100,000 | 2.594 | 38.55M | 2.059 | 48.56M | 65.740 | 25.34× | 31.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.227 | 3.50× |
| 1 | 5 | 0.241 | 0.843 | 3.49× |
| 1 | 10 | 0.493 | 1.953 | 3.96× |
| 10 | 1 | 0.051 | 0.177 | 3.50× |
| 10 | 5 | 0.268 | 0.884 | 3.30× |
| 10 | 10 | 0.526 | 2.003 | 3.81× |
| 100 | 1 | 0.064 | 0.228 | 3.57× |
| 100 | 5 | 0.247 | 1.145 | 4.63× |
| 100 | 10 | 0.529 | 2.502 | 4.73× |
| 1,000 | 1 | 0.079 | 0.963 | 12.19× |
| 1,000 | 5 | 0.248 | 4.187 | 16.88× |
| 1,000 | 10 | 0.547 | 8.469 | 15.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
