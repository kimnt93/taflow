# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.07M | 0.015 | 68.54M | 0.631 | 34.12× | 43.25× |
| 10,000 | 0.080 | 125.71M | 0.075 | 133.14M | 4.833 | 60.76× | 64.35× |
| 100,000 | 0.699 | 143.04M | 0.657 | 152.28M | 53.171 | 76.05× | 80.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.297 | 3.14× |
| 1 | 5 | 0.397 | 1.228 | 3.09× |
| 1 | 10 | 0.578 | 2.455 | 4.24× |
| 10 | 1 | 0.058 | 0.245 | 4.22× |
| 10 | 5 | 0.274 | 1.423 | 5.19× |
| 10 | 10 | 0.578 | 2.580 | 4.47× |
| 100 | 1 | 0.064 | 0.298 | 4.68× |
| 100 | 5 | 0.301 | 1.638 | 5.44× |
| 100 | 10 | 0.602 | 3.061 | 5.09× |
| 1,000 | 1 | 0.066 | 0.928 | 14.07× |
| 1,000 | 5 | 0.296 | 4.103 | 13.87× |
| 1,000 | 10 | 0.582 | 8.205 | 14.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
