# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.46M | 0.044 | 22.62M | 0.033 | 0.61× | 0.75× |
| 10,000 | 0.338 | 29.55M | 0.327 | 30.55M | 0.083 | 0.24× | 0.25× |
| 100,000 | 3.170 | 31.55M | 3.178 | 31.46M | 0.545 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.165 | 0.94× |
| 1 | 5 | 0.371 | 0.484 | 1.30× |
| 1 | 10 | 0.659 | 0.944 | 1.43× |
| 10 | 1 | 0.068 | 0.092 | 1.35× |
| 10 | 5 | 0.322 | 0.452 | 1.40× |
| 10 | 10 | 0.636 | 0.928 | 1.46× |
| 100 | 1 | 0.072 | 0.092 | 1.26× |
| 100 | 5 | 0.347 | 0.473 | 1.36× |
| 100 | 10 | 0.713 | 0.968 | 1.36× |
| 1,000 | 1 | 0.109 | 0.098 | 0.90× |
| 1,000 | 5 | 0.314 | 0.468 | 1.49× |
| 1,000 | 10 | 0.659 | 0.994 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
