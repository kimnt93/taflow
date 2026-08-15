# PositiveVolumeIndex benchmark (`PVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.67M | 0.004 | 228.55M | 0.206 | 34.31× | 47.05× |
| 10,000 | 0.057 | 175.50M | 0.054 | 186.45M | 0.769 | 13.50× | 14.35× |
| 100,000 | 0.584 | 171.30M | 0.555 | 180.03M | 6.693 | 11.46× | 12.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.276 | 3.34× |
| 1 | 5 | 0.323 | 1.021 | 3.16× |
| 1 | 10 | 0.396 | 2.301 | 5.81× |
| 10 | 1 | 0.048 | 0.210 | 4.33× |
| 10 | 5 | 0.191 | 1.257 | 6.58× |
| 10 | 10 | 0.433 | 2.251 | 5.19× |
| 100 | 1 | 0.057 | 0.209 | 3.65× |
| 100 | 5 | 0.195 | 1.368 | 7.01× |
| 100 | 10 | 0.439 | 2.297 | 5.24× |
| 1,000 | 1 | 0.049 | 0.267 | 5.47× |
| 1,000 | 5 | 0.198 | 1.697 | 8.56× |
| 1,000 | 10 | 0.422 | 2.937 | 6.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
