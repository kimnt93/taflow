# LowerLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.33M | 0.006 | 164.14M | nan | — | — |
| 10,000 | 0.032 | 314.65M | 0.029 | 343.29M | nan | — | — |
| 100,000 | 0.272 | 367.88M | 0.251 | 398.15M | nan | — | — |
| 1,000,000 | 3.026 | 330.50M | 2.940 | 340.11M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | nan | — |
| 1 | 5 | 0.463 | nan | — |
| 1 | 10 | 0.482 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.212 | nan | — |
| 10 | 10 | 0.480 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.219 | nan | — |
| 100 | 10 | 0.470 | nan | — |
| 1,000 | 1 | 0.050 | nan | — |
| 1,000 | 5 | 0.244 | nan | — |
| 1,000 | 10 | 0.491 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
