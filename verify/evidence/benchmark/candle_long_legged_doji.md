# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.65M | 0.003 | 378.59M | 0.033 | 5.44× | 12.59× |
| 10,000 | 0.045 | 221.85M | 0.039 | 253.68M | 0.094 | 2.08× | 2.38× |
| 100,000 | 0.494 | 202.44M | 0.516 | 193.84M | 0.628 | 1.27× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.118 | 1.65× |
| 1 | 5 | 0.237 | 0.464 | 1.96× |
| 1 | 10 | 0.362 | 0.908 | 2.51× |
| 10 | 1 | 0.040 | 0.088 | 2.19× |
| 10 | 5 | 0.191 | 0.425 | 2.22× |
| 10 | 10 | 0.367 | 0.894 | 2.43× |
| 100 | 1 | 0.038 | 0.093 | 2.45× |
| 100 | 5 | 0.178 | 0.428 | 2.40× |
| 100 | 10 | 0.380 | 0.883 | 2.32× |
| 1,000 | 1 | 0.048 | 0.092 | 1.93× |
| 1,000 | 5 | 0.194 | 0.469 | 2.41× |
| 1,000 | 10 | 0.425 | 0.954 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
