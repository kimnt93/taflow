# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.55M | 0.057 | 17.61M | nan | — | — |
| 10,000 | 0.552 | 18.13M | 0.546 | 18.33M | nan | — | — |
| 100,000 | 5.599 | 17.86M | 5.416 | 18.46M | nan | — | — |
| 1,000,000 | 55.114 | 18.14M | 55.822 | 17.91M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | nan | — |
| 1 | 5 | 0.329 | nan | — |
| 1 | 10 | 0.493 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.239 | nan | — |
| 10 | 10 | 0.451 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.226 | nan | — |
| 100 | 10 | 0.499 | nan | — |
| 1,000 | 1 | 0.108 | nan | — |
| 1,000 | 5 | 0.238 | nan | — |
| 1,000 | 10 | 0.509 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
