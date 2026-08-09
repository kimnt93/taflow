# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.10M | 0.015 | 65.10M | nan | — | — |
| 10,000 | 0.120 | 83.44M | 0.106 | 93.91M | nan | — | — |
| 100,000 | 1.214 | 82.35M | 1.065 | 93.86M | nan | — | — |
| 1,000,000 | 12.152 | 82.29M | 10.861 | 92.08M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | nan | — |
| 1 | 5 | 0.335 | nan | — |
| 1 | 10 | 0.494 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.227 | nan | — |
| 10 | 10 | 0.471 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.223 | nan | — |
| 100 | 10 | 0.512 | nan | — |
| 1,000 | 1 | 0.072 | nan | — |
| 1,000 | 5 | 0.299 | nan | — |
| 1,000 | 10 | 0.634 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
