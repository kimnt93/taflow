# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.10M | 0.041 | 24.32M | nan | — | — |
| 10,000 | 0.413 | 24.19M | 0.399 | 25.04M | nan | — | — |
| 100,000 | 4.299 | 23.26M | 4.078 | 24.52M | nan | — | — |
| 1,000,000 | 51.672 | 19.35M | 40.355 | 24.78M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | nan | — |
| 1 | 5 | 0.272 | nan | — |
| 1 | 10 | 0.655 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.243 | nan | — |
| 10 | 10 | 0.535 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.259 | nan | — |
| 100 | 10 | 0.609 | nan | — |
| 1,000 | 1 | 0.114 | nan | — |
| 1,000 | 5 | 0.287 | nan | — |
| 1,000 | 10 | 0.580 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
