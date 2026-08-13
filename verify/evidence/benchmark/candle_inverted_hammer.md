# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.103 | 9.73M | 0.095 | 10.57M | 0.039 | 0.38× | 0.41× |
| 10,000 | 0.897 | 11.15M | 0.833 | 12.01M | 0.159 | 0.18× | 0.19× |
| 100,000 | 8.441 | 11.85M | 8.697 | 11.50M | 1.326 | 0.16× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.104 | 0.96× |
| 1 | 5 | 0.464 | 0.450 | 0.97× |
| 1 | 10 | 0.653 | 0.902 | 1.38× |
| 10 | 1 | 0.069 | 0.087 | 1.26× |
| 10 | 5 | 0.315 | 0.443 | 1.41× |
| 10 | 10 | 0.647 | 0.893 | 1.38× |
| 100 | 1 | 0.077 | 0.095 | 1.23× |
| 100 | 5 | 0.307 | 0.419 | 1.36× |
| 100 | 10 | 0.652 | 0.948 | 1.45× |
| 1,000 | 1 | 0.166 | 0.108 | 0.65× |
| 1,000 | 5 | 0.381 | 0.564 | 1.48× |
| 1,000 | 10 | 0.691 | 1.021 | 1.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
