# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.32M | 0.008 | 121.30M | nan | — | — |
| 10,000 | 0.055 | 182.06M | 0.053 | 187.84M | nan | — | — |
| 100,000 | 0.572 | 174.96M | 0.494 | 202.57M | nan | — | — |
| 1,000,000 | 8.812 | 113.48M | 4.817 | 207.59M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | nan | — |
| 1 | 5 | 0.260 | nan | — |
| 1 | 10 | 0.543 | nan | — |
| 10 | 1 | 0.062 | nan | — |
| 10 | 5 | 0.264 | nan | — |
| 10 | 10 | 0.508 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.272 | nan | — |
| 100 | 10 | 0.507 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.254 | nan | — |
| 1,000 | 10 | 0.668 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
