# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.02M | 0.004 | 285.10M | 0.040 | 6.05× | 11.34× |
| 10,000 | 0.058 | 173.60M | 0.050 | 198.17M | 0.113 | 1.96× | 2.24× |
| 100,000 | 0.826 | 121.04M | 0.845 | 118.29M | 0.799 | 0.97× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.139 | 2.08× |
| 1 | 5 | 0.300 | 0.466 | 1.56× |
| 1 | 10 | 0.392 | 0.957 | 2.44× |
| 10 | 1 | 0.041 | 0.094 | 2.30× |
| 10 | 5 | 0.194 | 0.454 | 2.34× |
| 10 | 10 | 0.374 | 0.934 | 2.50× |
| 100 | 1 | 0.042 | 0.098 | 2.35× |
| 100 | 5 | 0.189 | 0.455 | 2.41× |
| 100 | 10 | 0.381 | 0.953 | 2.50× |
| 1,000 | 1 | 0.048 | 0.102 | 2.15× |
| 1,000 | 5 | 0.190 | 0.512 | 2.69× |
| 1,000 | 10 | 0.387 | 1.045 | 2.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
