# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.63M | 0.046 | 21.61M | nan | — | — |
| 10,000 | 0.436 | 22.94M | 0.432 | 23.16M | nan | — | — |
| 100,000 | 4.392 | 22.77M | 4.353 | 22.97M | nan | — | — |
| 1,000,000 | 52.514 | 19.04M | 43.092 | 23.21M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | nan | — |
| 1 | 5 | 0.400 | nan | — |
| 1 | 10 | 0.520 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.235 | nan | — |
| 10 | 10 | 0.484 | nan | — |
| 100 | 1 | 0.055 | nan | — |
| 100 | 5 | 0.244 | nan | — |
| 100 | 10 | 0.528 | nan | — |
| 1,000 | 1 | 0.100 | nan | — |
| 1,000 | 5 | 0.288 | nan | — |
| 1,000 | 10 | 0.581 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
