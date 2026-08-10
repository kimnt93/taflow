# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.81M | 0.122 | 8.19M | 0.322 | 2.52× | 2.64× |
| 10,000 | 1.253 | 7.98M | 1.233 | 8.11M | 1.754 | 1.40× | 1.42× |
| 100,000 | 12.456 | 8.03M | 12.555 | 7.97M | 16.022 | 1.29× | 1.28× |
| 1,000,000 | 127.035 | 7.87M | 124.362 | 8.04M | 157.539 | 1.24× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.556 | 4.87× |
| 1 | 5 | 0.239 | 1.198 | 5.02× |
| 1 | 10 | 0.463 | 2.583 | 5.57× |
| 10 | 1 | 0.056 | 0.229 | 4.12× |
| 10 | 5 | 0.242 | 1.173 | 4.84× |
| 10 | 10 | 0.470 | 2.425 | 5.16× |
| 100 | 1 | 0.060 | 0.256 | 4.25× |
| 100 | 5 | 0.246 | 1.425 | 5.80× |
| 100 | 10 | 0.490 | 2.649 | 5.41× |
| 1,000 | 1 | 0.190 | 0.416 | 2.19× |
| 1,000 | 5 | 0.374 | 2.264 | 6.05× |
| 1,000 | 10 | 0.612 | 4.296 | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
