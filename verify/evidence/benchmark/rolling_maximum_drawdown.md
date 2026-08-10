# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.48M | 0.046 | 21.54M | 0.236 | 5.07× | 5.09× |
| 10,000 | 0.459 | 21.79M | 0.452 | 22.12M | 1.277 | 2.78× | 2.83× |
| 100,000 | 4.443 | 22.50M | 4.466 | 22.39M | 11.550 | 2.60× | 2.59× |
| 1,000,000 | 44.045 | 22.70M | 43.948 | 22.75M | 116.380 | 2.64× | 2.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.275 | 2.85× |
| 1 | 5 | 0.348 | 1.205 | 3.47× |
| 1 | 10 | 0.482 | 2.107 | 4.37× |
| 10 | 1 | 0.059 | 0.198 | 3.36× |
| 10 | 5 | 0.225 | 0.949 | 4.22× |
| 10 | 10 | 0.494 | 2.084 | 4.22× |
| 100 | 1 | 0.058 | 0.206 | 3.56× |
| 100 | 5 | 0.251 | 1.012 | 4.03× |
| 100 | 10 | 0.513 | 2.196 | 4.28× |
| 1,000 | 1 | 0.100 | 0.312 | 3.14× |
| 1,000 | 5 | 0.267 | 1.563 | 5.87× |
| 1,000 | 10 | 0.512 | 3.368 | 6.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
