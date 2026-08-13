# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.103 | 9.69M | 0.094 | 10.64M | 0.039 | 0.38× | 0.42× |
| 10,000 | 0.857 | 11.67M | 0.866 | 11.55M | 0.159 | 0.19× | 0.18× |
| 100,000 | 8.168 | 12.24M | 9.620 | 10.40M | 1.402 | 0.17× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.143 | 0.93× |
| 1 | 5 | 0.409 | 0.480 | 1.17× |
| 1 | 10 | 0.646 | 0.913 | 1.41× |
| 10 | 1 | 0.069 | 0.088 | 1.27× |
| 10 | 5 | 0.305 | 0.429 | 1.41× |
| 10 | 10 | 0.644 | 0.933 | 1.45× |
| 100 | 1 | 0.080 | 0.105 | 1.31× |
| 100 | 5 | 0.317 | 0.437 | 1.38× |
| 100 | 10 | 0.672 | 0.930 | 1.38× |
| 1,000 | 1 | 0.161 | 0.107 | 0.66× |
| 1,000 | 5 | 0.357 | 0.497 | 1.39× |
| 1,000 | 10 | 0.683 | 1.031 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
