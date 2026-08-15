# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.11M | 0.051 | 19.63M | 0.088 | 1.67× | 1.72× |
| 10,000 | 0.519 | 19.25M | 0.518 | 19.31M | 0.545 | 1.05× | 1.05× |
| 100,000 | 5.250 | 19.05M | 5.064 | 19.75M | 5.437 | 1.04× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.117 | 1.67× |
| 1 | 5 | 0.311 | 0.512 | 1.65× |
| 1 | 10 | 0.443 | 1.057 | 2.39× |
| 10 | 1 | 0.044 | 0.102 | 2.34× |
| 10 | 5 | 0.189 | 0.485 | 2.57× |
| 10 | 10 | 0.393 | 1.051 | 2.67× |
| 100 | 1 | 0.050 | 0.102 | 2.04× |
| 100 | 5 | 0.195 | 0.501 | 2.56× |
| 100 | 10 | 0.456 | 1.064 | 2.33× |
| 1,000 | 1 | 0.105 | 0.162 | 1.54× |
| 1,000 | 5 | 0.226 | 0.765 | 3.38× |
| 1,000 | 10 | 0.441 | 1.603 | 3.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
