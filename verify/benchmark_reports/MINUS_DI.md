# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.13M | 0.015 | 64.88M | 0.048 | 2.78× | 3.10× |
| 10,000 | 0.124 | 80.49M | 0.114 | 87.71M | 0.109 | 0.87× | 0.95× |
| 100,000 | 1.162 | 86.09M | 1.065 | 93.93M | 0.760 | 0.65× | 0.71× |
| 1,000,000 | 11.184 | 89.41M | 10.292 | 97.16M | 8.307 | 0.74× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.103 | 1.57× |
| 1 | 5 | 0.311 | 0.535 | 1.72× |
| 1 | 10 | 0.559 | 1.092 | 1.95× |
| 10 | 1 | 0.054 | 0.099 | 1.84× |
| 10 | 5 | 0.285 | 0.513 | 1.80× |
| 10 | 10 | 0.572 | 1.099 | 1.92× |
| 100 | 1 | 0.053 | 0.101 | 1.91× |
| 100 | 5 | 0.264 | 0.494 | 1.87× |
| 100 | 10 | 0.528 | 1.093 | 2.07× |
| 1,000 | 1 | 0.067 | 0.111 | 1.65× |
| 1,000 | 5 | 0.283 | 0.547 | 1.93× |
| 1,000 | 10 | 0.565 | 1.125 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
