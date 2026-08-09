# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.06M | 0.009 | 112.44M | 0.042 | 3.55× | 4.70× |
| 10,000 | 0.121 | 82.72M | 0.120 | 83.16M | 0.175 | 1.45× | 1.45× |
| 100,000 | 1.296 | 77.16M | 1.220 | 81.97M | 1.411 | 1.09× | 1.16× |
| 1,000,000 | 12.665 | 78.96M | 12.482 | 80.11M | 13.877 | 1.10× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.124 | 1.13× |
| 1 | 5 | 0.377 | 0.655 | 1.74× |
| 1 | 10 | 0.517 | 0.948 | 1.83× |
| 10 | 1 | 0.052 | 0.091 | 1.74× |
| 10 | 5 | 0.241 | 0.439 | 1.82× |
| 10 | 10 | 0.493 | 0.949 | 1.92× |
| 100 | 1 | 0.056 | 0.094 | 1.69× |
| 100 | 5 | 0.285 | 0.498 | 1.75× |
| 100 | 10 | 0.528 | 0.980 | 1.86× |
| 1,000 | 1 | 0.068 | 0.108 | 1.59× |
| 1,000 | 5 | 0.254 | 0.514 | 2.03× |
| 1,000 | 10 | 0.561 | 1.113 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
