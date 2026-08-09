# PositiveVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.28M | 0.006 | 171.86M | nan | — | — |
| 10,000 | 0.060 | 166.21M | 0.055 | 181.67M | nan | — | — |
| 100,000 | 0.559 | 178.99M | 0.526 | 190.04M | nan | — | — |
| 1,000,000 | 5.729 | 174.54M | 5.325 | 187.80M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | nan | — |
| 1 | 5 | 0.406 | nan | — |
| 1 | 10 | 0.501 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.217 | nan | — |
| 10 | 10 | 0.470 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.213 | nan | — |
| 100 | 10 | 0.433 | nan | — |
| 1,000 | 1 | 0.056 | nan | — |
| 1,000 | 5 | 0.216 | nan | — |
| 1,000 | 10 | 0.491 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
