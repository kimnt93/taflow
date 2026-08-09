# UlcerIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.27M | 0.045 | 22.11M | nan | — | — |
| 10,000 | 0.430 | 23.27M | 0.429 | 23.32M | nan | — | — |
| 100,000 | 4.185 | 23.89M | 4.270 | 23.42M | nan | — | — |
| 1,000,000 | 43.105 | 23.20M | 38.499 | 25.97M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | nan | — |
| 1 | 5 | 0.432 | nan | — |
| 1 | 10 | 0.460 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.226 | nan | — |
| 10 | 10 | 0.453 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.220 | nan | — |
| 100 | 10 | 0.495 | nan | — |
| 1,000 | 1 | 0.087 | nan | — |
| 1,000 | 5 | 0.224 | nan | — |
| 1,000 | 10 | 0.501 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
