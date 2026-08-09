# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.45M | 0.006 | 166.63M | nan | — | — |
| 10,000 | 0.034 | 294.45M | 0.030 | 332.23M | nan | — | — |
| 100,000 | 0.339 | 294.61M | 0.251 | 398.22M | nan | — | — |
| 1,000,000 | 3.340 | 299.41M | 2.895 | 345.48M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | nan | — |
| 1 | 5 | 0.328 | nan | — |
| 1 | 10 | 0.488 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.217 | nan | — |
| 10 | 10 | 0.460 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.215 | nan | — |
| 100 | 10 | 0.482 | nan | — |
| 1,000 | 1 | 0.049 | nan | — |
| 1,000 | 5 | 0.233 | nan | — |
| 1,000 | 10 | 0.474 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
