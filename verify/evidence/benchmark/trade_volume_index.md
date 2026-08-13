# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.24M | 0.037 | 26.94M | 0.198 | 4.40× | 5.33× |
| 10,000 | 0.299 | 33.49M | 0.287 | 34.89M | 0.780 | 2.61× | 2.72× |
| 100,000 | 2.735 | 36.56M | 2.763 | 36.20M | 6.570 | 2.40× | 2.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.178 | 0.284 | 1.60× |
| 1 | 5 | 0.364 | 1.121 | 3.08× |
| 1 | 10 | 0.648 | 2.218 | 3.42× |
| 10 | 1 | 0.073 | 0.211 | 2.88× |
| 10 | 5 | 0.301 | 1.226 | 4.08× |
| 10 | 10 | 0.634 | 2.249 | 3.55× |
| 100 | 1 | 0.068 | 0.218 | 3.19× |
| 100 | 5 | 0.298 | 1.302 | 4.37× |
| 100 | 10 | 0.665 | 2.398 | 3.60× |
| 1,000 | 1 | 0.112 | 0.278 | 2.49× |
| 1,000 | 5 | 0.325 | 1.595 | 4.90× |
| 1,000 | 10 | 0.649 | 2.942 | 4.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
