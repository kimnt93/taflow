# OrderBlock benchmark (`causal dual-scale order blocks` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.081 | 12.34M | 0.072 | 13.81M | 10.322 | 127.40× | 142.56× |
| 10,000 | 0.885 | 11.30M | 0.829 | 12.07M | 119.098 | 134.55× | 143.72× |
| 100,000 | 9.518 | 10.51M | 8.922 | 11.21M | 1295.685 | 136.12× | 145.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.201 | 0.213 | 1.06× |
| 1 | 5 | 0.248 | 0.879 | 3.55× |
| 1 | 10 | 0.459 | 1.669 | 3.64× |
| 10 | 1 | 0.050 | 0.177 | 3.52× |
| 10 | 5 | 0.209 | 0.866 | 4.14× |
| 10 | 10 | 0.468 | 1.778 | 3.80× |
| 100 | 1 | 0.055 | 0.640 | 11.61× |
| 100 | 5 | 0.215 | 3.266 | 15.21× |
| 100 | 10 | 0.436 | 6.664 | 15.28× |
| 1,000 | 1 | 0.128 | 10.096 | 78.83× |
| 1,000 | 5 | 0.405 | 57.529 | 142.17× |
| 1,000 | 10 | 0.750 | 128.958 | 172.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
