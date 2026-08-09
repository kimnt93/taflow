# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.26M | 0.005 | 214.15M | nan | — | — |
| 10,000 | 0.023 | 427.45M | 0.020 | 489.35M | nan | — | — |
| 100,000 | 0.198 | 504.70M | 0.178 | 561.99M | nan | — | — |
| 1,000,000 | 2.390 | 418.46M | 1.833 | 545.60M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | nan | — |
| 1 | 5 | 0.366 | nan | — |
| 1 | 10 | 0.481 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.216 | nan | — |
| 10 | 10 | 0.446 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.214 | nan | — |
| 100 | 10 | 0.491 | nan | — |
| 1,000 | 1 | 0.060 | nan | — |
| 1,000 | 5 | 0.259 | nan | — |
| 1,000 | 10 | 0.528 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
