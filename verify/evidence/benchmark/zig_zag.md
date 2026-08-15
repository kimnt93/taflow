# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.93M | 0.006 | 155.66M | 0.501 | 58.61× | 78.03× |
| 10,000 | 0.077 | 129.14M | 0.069 | 143.99M | 3.843 | 49.63× | 55.34× |
| 100,000 | 0.842 | 118.83M | 0.737 | 135.75M | 39.162 | 46.54× | 53.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.262 | 3.08× |
| 1 | 5 | 0.256 | 1.067 | 4.17× |
| 1 | 10 | 0.427 | 2.362 | 5.54× |
| 10 | 1 | 0.045 | 0.216 | 4.79× |
| 10 | 5 | 0.192 | 1.252 | 6.52× |
| 10 | 10 | 0.398 | 2.367 | 5.94× |
| 100 | 1 | 0.052 | 0.253 | 4.83× |
| 100 | 5 | 0.205 | 1.411 | 6.89× |
| 100 | 10 | 0.476 | 2.764 | 5.81× |
| 1,000 | 1 | 0.067 | 0.725 | 10.87× |
| 1,000 | 5 | 0.227 | 3.183 | 14.04× |
| 1,000 | 10 | 0.436 | 6.427 | 14.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
