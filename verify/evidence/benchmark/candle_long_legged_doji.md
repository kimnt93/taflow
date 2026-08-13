# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.19M | 0.055 | 18.06M | 0.033 | 0.50× | 0.59× |
| 10,000 | 0.546 | 18.30M | 0.446 | 22.40M | 0.088 | 0.16× | 0.20× |
| 100,000 | 4.350 | 22.99M | 4.335 | 23.07M | 0.755 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.100 | 0.87× |
| 1 | 5 | 0.363 | 0.455 | 1.25× |
| 1 | 10 | 0.626 | 0.940 | 1.50× |
| 10 | 1 | 0.068 | 0.092 | 1.36× |
| 10 | 5 | 0.314 | 0.418 | 1.33× |
| 10 | 10 | 0.640 | 0.897 | 1.40× |
| 100 | 1 | 0.071 | 0.091 | 1.28× |
| 100 | 5 | 0.329 | 0.479 | 1.46× |
| 100 | 10 | 0.670 | 0.878 | 1.31× |
| 1,000 | 1 | 0.122 | 0.093 | 0.77× |
| 1,000 | 5 | 0.297 | 0.470 | 1.58× |
| 1,000 | 10 | 0.652 | 0.993 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
