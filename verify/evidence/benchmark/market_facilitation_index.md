# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.38M | 0.003 | 317.39M | 0.186 | 31.15× | 59.06× |
| 10,000 | 0.024 | 423.60M | 0.019 | 530.13M | 0.980 | 41.53× | 51.97× |
| 100,000 | 0.215 | 465.11M | 0.178 | 562.68M | 9.500 | 44.19× | 53.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.224 | 1.44× |
| 1 | 5 | 0.308 | 1.202 | 3.91× |
| 1 | 10 | 0.392 | 1.944 | 4.96× |
| 10 | 1 | 0.048 | 0.174 | 3.67× |
| 10 | 5 | 0.213 | 0.836 | 3.92× |
| 10 | 10 | 0.418 | 1.892 | 4.52× |
| 100 | 1 | 0.054 | 0.172 | 3.18× |
| 100 | 5 | 0.201 | 0.889 | 4.43× |
| 100 | 10 | 0.431 | 1.969 | 4.56× |
| 1,000 | 1 | 0.044 | 0.263 | 5.95× |
| 1,000 | 5 | 0.240 | 1.312 | 5.47× |
| 1,000 | 10 | 0.428 | 2.640 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
