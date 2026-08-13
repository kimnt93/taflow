# SessionVolumeLevels benchmark (`anchored volume levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.524 | 1.91M | 0.513 | 1.95M | 13.384 | 25.55× | 26.10× |
| 10,000 | 5.405 | 1.85M | 5.187 | 1.93M | 139.592 | 25.82× | 26.91× |
| 100,000 | 52.165 | 1.92M | 51.814 | 1.93M | 1389.292 | 26.63× | 26.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.189 | 1.54× |
| 1 | 5 | 0.451 | 0.755 | 1.67× |
| 1 | 10 | 0.689 | 1.361 | 1.98× |
| 10 | 1 | 0.090 | 0.293 | 3.25× |
| 10 | 5 | 0.362 | 1.495 | 4.13× |
| 10 | 10 | 0.758 | 3.095 | 4.08× |
| 100 | 1 | 0.141 | 1.831 | 12.99× |
| 100 | 5 | 0.671 | 10.093 | 15.04× |
| 100 | 10 | 1.346 | 19.906 | 14.78× |
| 1,000 | 1 | 0.615 | 13.976 | 22.72× |
| 1,000 | 5 | 3.334 | 78.229 | 23.47× |
| 1,000 | 10 | 6.306 | 159.052 | 25.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
