# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.03M | 0.009 | 113.97M | 0.522 | 34.48× | 59.52× |
| 10,000 | 0.082 | 122.13M | 0.076 | 131.14M | 3.893 | 47.54× | 51.05× |
| 100,000 | 0.797 | 125.43M | 0.755 | 132.46M | 41.637 | 52.22× | 55.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.300 | 3.59× |
| 1 | 5 | 0.331 | 1.227 | 3.71× |
| 1 | 10 | 0.505 | 2.360 | 4.67× |
| 10 | 1 | 0.051 | 0.227 | 4.45× |
| 10 | 5 | 0.248 | 1.337 | 5.38× |
| 10 | 10 | 0.526 | 2.836 | 5.39× |
| 100 | 1 | 0.057 | 0.273 | 4.81× |
| 100 | 5 | 0.246 | 1.503 | 6.11× |
| 100 | 10 | 0.468 | 2.811 | 6.01× |
| 1,000 | 1 | 0.059 | 0.657 | 11.16× |
| 1,000 | 5 | 0.243 | 3.191 | 13.15× |
| 1,000 | 10 | 0.536 | 6.373 | 11.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
