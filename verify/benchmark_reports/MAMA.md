# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.75M | 0.057 | 17.54M | 0.090 | 1.51× | 1.58× |
| 10,000 | 0.561 | 17.83M | 0.597 | 16.74M | 0.565 | 1.01× | 0.95× |
| 100,000 | 5.830 | 17.15M | 6.257 | 15.98M | 5.463 | 0.94× | 0.87× |
| 1,000,000 | 59.897 | 16.70M | 56.708 | 17.63M | 54.319 | 0.91× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.126 | 1.30× |
| 1 | 5 | 0.258 | 0.517 | 2.01× |
| 1 | 10 | 0.500 | 1.070 | 2.14× |
| 10 | 1 | 0.052 | 0.100 | 1.93× |
| 10 | 5 | 0.240 | 0.506 | 2.11× |
| 10 | 10 | 0.493 | 1.054 | 2.14× |
| 100 | 1 | 0.061 | 0.116 | 1.91× |
| 100 | 5 | 0.246 | 0.528 | 2.15× |
| 100 | 10 | 0.501 | 1.069 | 2.13× |
| 1,000 | 1 | 0.113 | 0.162 | 1.44× |
| 1,000 | 5 | 0.280 | 0.824 | 2.95× |
| 1,000 | 10 | 0.543 | 1.670 | 3.08× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.396 | 0.348 | 2.88M | 5716.523 | 16445.07× | 113.80× |
| 100,000 | 10 | 2.824 | 2.237 | 4.47M | 5485.663 | 2452.37× | 17.95× |
| 100,000 | 1,000 | 166.342 | 164.988 | 6.06M | 5553.177 | 33.66× | 0.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.93M | 17.03M | 1.00× | 1.54M | 1.66M | 1.00× | 17.09M |
| 5 | 52.27M | 55.06M | 3.23× | 1.61M | 1.52M | 0.91× | 16.18M |
| 10 | 74.73M | 103.86M | 6.10× | 1.54M | 1.64M | 0.99× | 17.46M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
