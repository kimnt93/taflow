# CumulativeCount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 194.34M | 0.004 | 252.00M | nan | — | — |
| 10,000 | 0.023 | 441.52M | 0.021 | 483.48M | nan | — | — |
| 100,000 | 0.213 | 469.18M | 0.180 | 555.77M | nan | — | — |
| 1,000,000 | 2.363 | 423.27M | 1.884 | 530.65M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | nan | — |
| 1 | 5 | 0.315 | nan | — |
| 1 | 10 | 0.484 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.243 | nan | — |
| 10 | 10 | 0.567 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.224 | nan | — |
| 100 | 10 | 0.490 | nan | — |
| 1,000 | 1 | 0.049 | nan | — |
| 1,000 | 5 | 0.216 | nan | — |
| 1,000 | 10 | 0.484 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
