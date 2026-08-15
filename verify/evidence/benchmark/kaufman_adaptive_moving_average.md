# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 212.52M | 0.004 | 258.79M | 0.035 | 7.34× | 8.94× |
| 10,000 | 0.032 | 313.75M | 0.029 | 348.16M | 0.063 | 1.96× | 2.18× |
| 100,000 | 0.306 | 326.85M | 0.283 | 353.55M | 0.328 | 1.07× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.120 | 0.90× |
| 1 | 5 | 0.220 | 0.448 | 2.04× |
| 1 | 10 | 0.377 | 0.924 | 2.45× |
| 10 | 1 | 0.049 | 0.110 | 2.26× |
| 10 | 5 | 0.193 | 0.453 | 2.35× |
| 10 | 10 | 0.401 | 0.935 | 2.33× |
| 100 | 1 | 0.045 | 0.088 | 1.95× |
| 100 | 5 | 0.196 | 0.457 | 2.33× |
| 100 | 10 | 0.410 | 0.937 | 2.28× |
| 1,000 | 1 | 0.042 | 0.093 | 2.19× |
| 1,000 | 5 | 0.196 | 0.447 | 2.28× |
| 1,000 | 10 | 0.397 | 1.013 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
