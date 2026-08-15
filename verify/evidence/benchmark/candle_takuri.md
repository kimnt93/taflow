# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.86M | 0.003 | 328.77M | 0.039 | 5.64× | 12.97× |
| 10,000 | 0.049 | 204.82M | 0.041 | 243.28M | 0.114 | 2.34× | 2.78× |
| 100,000 | 0.584 | 171.14M | 0.540 | 185.22M | 0.807 | 1.38× | 1.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.105 | 1.76× |
| 1 | 5 | 0.297 | 0.471 | 1.59× |
| 1 | 10 | 0.416 | 0.916 | 2.20× |
| 10 | 1 | 0.043 | 0.095 | 2.22× |
| 10 | 5 | 0.177 | 0.439 | 2.48× |
| 10 | 10 | 0.467 | 0.925 | 1.98× |
| 100 | 1 | 0.046 | 0.086 | 1.86× |
| 100 | 5 | 0.191 | 0.449 | 2.35× |
| 100 | 10 | 0.388 | 0.951 | 2.45× |
| 1,000 | 1 | 0.051 | 0.101 | 1.96× |
| 1,000 | 5 | 0.209 | 0.473 | 2.27× |
| 1,000 | 10 | 0.431 | 1.057 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
