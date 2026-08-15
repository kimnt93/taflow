# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.05M | 0.003 | 394.97M | 0.031 | 5.50× | 12.27× |
| 10,000 | 0.048 | 206.51M | 0.045 | 222.79M | 0.081 | 1.68× | 1.81× |
| 100,000 | 0.505 | 197.83M | 0.496 | 201.52M | 0.604 | 1.19× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.127 | 2.00× |
| 1 | 5 | 0.239 | 0.470 | 1.97× |
| 1 | 10 | 0.358 | 0.847 | 2.36× |
| 10 | 1 | 0.041 | 0.087 | 2.16× |
| 10 | 5 | 0.172 | 0.412 | 2.39× |
| 10 | 10 | 0.381 | 0.879 | 2.31× |
| 100 | 1 | 0.041 | 0.086 | 2.11× |
| 100 | 5 | 0.193 | 0.411 | 2.13× |
| 100 | 10 | 0.392 | 0.846 | 2.16× |
| 1,000 | 1 | 0.046 | 0.095 | 2.04× |
| 1,000 | 5 | 0.193 | 0.455 | 2.36× |
| 1,000 | 10 | 0.388 | 0.960 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
