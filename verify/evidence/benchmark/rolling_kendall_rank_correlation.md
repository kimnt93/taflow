# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 5.390 | 185.52K | 5.698 | 175.49K | 0.802 | 0.15× | 0.14× |
| 10,000 | 55.193 | 181.18K | 56.108 | 178.23K | 6.742 | 0.12× | 0.12× |
| 100,000 | 567.419 | 176.24K | 559.525 | 178.72K | 67.638 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.257 | 1.70× |
| 1 | 5 | 0.414 | 1.251 | 3.02× |
| 1 | 10 | 0.646 | 2.340 | 3.62× |
| 10 | 1 | 0.077 | 0.216 | 2.81× |
| 10 | 5 | 0.308 | 1.224 | 3.97× |
| 10 | 10 | 0.635 | 2.247 | 3.54× |
| 100 | 1 | 0.545 | 0.279 | 0.51× |
| 100 | 5 | 0.884 | 1.845 | 2.09× |
| 100 | 10 | 1.308 | 2.874 | 2.20× |
| 1,000 | 1 | 5.880 | 0.962 | 0.16× |
| 1,000 | 5 | 9.081 | 4.955 | 0.55× |
| 1,000 | 10 | 11.015 | 9.847 | 0.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
