# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.288 | 3.47M | 0.288 | 3.47M | 0.050 | 0.17× | 0.17× |
| 10,000 | 3.185 | 3.14M | 2.844 | 3.52M | 0.170 | 0.05× | 0.06× |
| 100,000 | 27.932 | 3.58M | 27.197 | 3.68M | 1.448 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.128 | 1.07× |
| 1 | 5 | 0.401 | 0.476 | 1.19× |
| 1 | 10 | 0.610 | 0.941 | 1.54× |
| 10 | 1 | 0.066 | 0.098 | 1.49× |
| 10 | 5 | 0.281 | 0.427 | 1.52× |
| 10 | 10 | 0.580 | 0.982 | 1.69× |
| 100 | 1 | 0.094 | 0.092 | 0.98× |
| 100 | 5 | 0.318 | 0.462 | 1.45× |
| 100 | 10 | 0.605 | 0.921 | 1.52× |
| 1,000 | 1 | 0.354 | 0.111 | 0.31× |
| 1,000 | 5 | 0.577 | 0.525 | 0.91× |
| 1,000 | 10 | 0.954 | 1.218 | 1.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
