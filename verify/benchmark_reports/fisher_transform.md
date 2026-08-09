# FisherTransform benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.04M | 0.037 | 26.89M | nan | — | — |
| 10,000 | 0.384 | 26.05M | 0.391 | 25.59M | nan | — | — |
| 100,000 | 3.756 | 26.62M | 3.787 | 26.40M | nan | — | — |
| 1,000,000 | 38.474 | 25.99M | 37.759 | 26.48M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | nan | — |
| 1 | 5 | 0.297 | nan | — |
| 1 | 10 | 0.496 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.235 | nan | — |
| 10 | 10 | 0.474 | nan | — |
| 100 | 1 | 0.056 | nan | — |
| 100 | 5 | 0.241 | nan | — |
| 100 | 10 | 0.509 | nan | — |
| 1,000 | 1 | 0.090 | nan | — |
| 1,000 | 5 | 0.261 | nan | — |
| 1,000 | 10 | 0.558 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
