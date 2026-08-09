# RollingSortino benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.17M | 0.020 | 48.98M | nan | — | — |
| 10,000 | 0.180 | 55.63M | 0.176 | 56.92M | nan | — | — |
| 100,000 | 1.794 | 55.73M | 1.704 | 58.68M | nan | — | — |
| 1,000,000 | 18.454 | 54.19M | 17.087 | 58.53M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | nan | — |
| 1 | 5 | 0.367 | nan | — |
| 1 | 10 | 0.460 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.241 | nan | — |
| 10 | 10 | 0.515 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.270 | nan | — |
| 100 | 10 | 0.531 | nan | — |
| 1,000 | 1 | 0.066 | nan | — |
| 1,000 | 5 | 0.287 | nan | — |
| 1,000 | 10 | 0.520 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
