# EqualHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.83M | 0.046 | 21.69M | nan | — | — |
| 10,000 | 0.447 | 22.39M | 0.443 | 22.59M | nan | — | — |
| 100,000 | 4.457 | 22.44M | 4.293 | 23.29M | nan | — | — |
| 1,000,000 | 56.496 | 17.70M | 46.447 | 21.53M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | nan | — |
| 1 | 5 | 0.256 | nan | — |
| 1 | 10 | 0.469 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.229 | nan | — |
| 10 | 10 | 0.513 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.242 | nan | — |
| 100 | 10 | 0.556 | nan | — |
| 1,000 | 1 | 0.103 | nan | — |
| 1,000 | 5 | 0.277 | nan | — |
| 1,000 | 10 | 0.579 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
