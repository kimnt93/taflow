# VolumeWeightedMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.97M | 0.015 | 64.73M | nan | — | — |
| 10,000 | 0.131 | 76.32M | 0.122 | 81.64M | nan | — | — |
| 100,000 | 1.192 | 83.87M | 1.184 | 84.49M | nan | — | — |
| 1,000,000 | 12.128 | 82.46M | 11.947 | 83.71M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | nan | — |
| 1 | 5 | 0.460 | nan | — |
| 1 | 10 | 0.468 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.208 | nan | — |
| 10 | 10 | 0.473 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.222 | nan | — |
| 100 | 10 | 0.491 | nan | — |
| 1,000 | 1 | 0.059 | nan | — |
| 1,000 | 5 | 0.242 | nan | — |
| 1,000 | 10 | 0.512 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
