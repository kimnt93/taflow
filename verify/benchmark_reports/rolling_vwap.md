# RollingVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.25M | 0.026 | 37.96M | nan | — | — |
| 10,000 | 0.221 | 45.28M | 0.220 | 45.46M | nan | — | — |
| 100,000 | 2.127 | 47.02M | 2.105 | 47.51M | nan | — | — |
| 1,000,000 | 21.865 | 45.73M | 21.832 | 45.80M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | nan | — |
| 1 | 5 | 0.309 | nan | — |
| 1 | 10 | 0.533 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.255 | nan | — |
| 10 | 10 | 0.506 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.251 | nan | — |
| 100 | 10 | 0.517 | nan | — |
| 1,000 | 1 | 0.081 | nan | — |
| 1,000 | 5 | 0.283 | nan | — |
| 1,000 | 10 | 0.564 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
