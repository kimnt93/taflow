# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.859 | 1.16M | 0.752 | 1.33M | 0.860 | 1.00× | 1.14× |
| 10,000 | 7.969 | 1.25M | 8.024 | 1.25M | 7.728 | 0.97× | 0.96× |
| 100,000 | 79.395 | 1.26M | 76.999 | 1.30M | 74.302 | 0.94× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.376 | 5.15× |
| 1 | 5 | 0.317 | 1.288 | 4.07× |
| 1 | 10 | 0.490 | 2.397 | 4.89× |
| 10 | 1 | 0.060 | 0.230 | 3.82× |
| 10 | 5 | 0.227 | 1.285 | 5.66× |
| 10 | 10 | 0.517 | 2.671 | 5.17× |
| 100 | 1 | 0.116 | 0.279 | 2.40× |
| 100 | 5 | 0.291 | 1.689 | 5.81× |
| 100 | 10 | 0.626 | 3.103 | 4.96× |
| 1,000 | 1 | 0.833 | 0.985 | 1.18× |
| 1,000 | 5 | 1.197 | 5.125 | 4.28× |
| 1,000 | 10 | 2.045 | 10.236 | 5.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
