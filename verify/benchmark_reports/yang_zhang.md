# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.10M | 0.054 | 18.66M | nan | — | — |
| 10,000 | 0.489 | 20.44M | 0.484 | 20.68M | nan | — | — |
| 100,000 | 4.757 | 21.02M | 4.750 | 21.05M | nan | — | — |
| 1,000,000 | 48.142 | 20.77M | 46.037 | 21.72M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | nan | — |
| 1 | 5 | 0.284 | nan | — |
| 1 | 10 | 0.557 | nan | — |
| 10 | 1 | 0.055 | nan | — |
| 10 | 5 | 0.250 | nan | — |
| 10 | 10 | 0.534 | nan | — |
| 100 | 1 | 0.070 | nan | — |
| 100 | 5 | 0.254 | nan | — |
| 100 | 10 | 0.515 | nan | — |
| 1,000 | 1 | 0.121 | nan | — |
| 1,000 | 5 | 0.290 | nan | — |
| 1,000 | 10 | 0.597 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
