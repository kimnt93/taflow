# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.641 | 1.56M | 0.623 | 1.61M | 0.832 | 1.30× | 1.34× |
| 10,000 | 6.332 | 1.58M | 6.267 | 1.60M | 7.720 | 1.22× | 1.23× |
| 100,000 | 65.714 | 1.52M | 67.875 | 1.47M | 77.109 | 1.17× | 1.14× |
| 1,000,000 | 650.941 | 1.54M | 646.772 | 1.55M | 697.495 | 1.07× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.287 | 3.56× |
| 1 | 5 | 0.331 | 1.249 | 3.78× |
| 1 | 10 | 0.501 | 2.278 | 4.55× |
| 10 | 1 | 0.053 | 0.213 | 4.05× |
| 10 | 5 | 0.236 | 1.255 | 5.32× |
| 10 | 10 | 0.477 | 2.257 | 4.73× |
| 100 | 1 | 0.112 | 0.275 | 2.46× |
| 100 | 5 | 0.246 | 1.525 | 6.19× |
| 100 | 10 | 0.552 | 2.950 | 5.34× |
| 1,000 | 1 | 0.733 | 0.941 | 1.28× |
| 1,000 | 5 | 0.972 | 4.913 | 5.06× |
| 1,000 | 10 | 1.345 | 9.668 | 7.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
