# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.78M | 0.004 | 278.72M | 0.042 | 6.25× | 11.79× |
| 10,000 | 0.095 | 105.13M | 0.089 | 111.96M | 0.167 | 1.76× | 1.87× |
| 100,000 | 1.080 | 92.63M | 1.065 | 93.92M | 1.432 | 1.33× | 1.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.129 | 2.11× |
| 1 | 5 | 0.227 | 0.442 | 1.95× |
| 1 | 10 | 0.407 | 0.964 | 2.37× |
| 10 | 1 | 0.046 | 0.110 | 2.40× |
| 10 | 5 | 0.178 | 0.428 | 2.41× |
| 10 | 10 | 0.408 | 0.958 | 2.35× |
| 100 | 1 | 0.041 | 0.091 | 2.21× |
| 100 | 5 | 0.203 | 0.481 | 2.37× |
| 100 | 10 | 0.416 | 0.945 | 2.27× |
| 1,000 | 1 | 0.059 | 0.100 | 1.69× |
| 1,000 | 5 | 0.197 | 0.544 | 2.75× |
| 1,000 | 10 | 0.462 | 1.062 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
