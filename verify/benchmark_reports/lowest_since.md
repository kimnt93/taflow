# LowestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.40M | 0.006 | 157.58M | nan | — | — |
| 10,000 | 0.038 | 260.35M | 0.035 | 283.08M | nan | — | — |
| 100,000 | 0.340 | 294.28M | 0.332 | 300.89M | nan | — | — |
| 1,000,000 | 3.675 | 272.11M | 3.412 | 293.09M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | nan | — |
| 1 | 5 | 0.274 | nan | — |
| 1 | 10 | 0.497 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.276 | nan | — |
| 10 | 10 | 0.533 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.273 | nan | — |
| 100 | 10 | 0.610 | nan | — |
| 1,000 | 1 | 0.061 | nan | — |
| 1,000 | 5 | 0.299 | nan | — |
| 1,000 | 10 | 0.583 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
