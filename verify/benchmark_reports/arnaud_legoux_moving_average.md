# ArnaudLegouxMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.32M | 0.017 | 58.58M | nan | — | — |
| 10,000 | 0.150 | 66.50M | 0.144 | 69.22M | nan | — | — |
| 100,000 | 1.556 | 64.28M | 1.460 | 68.48M | nan | — | — |
| 1,000,000 | 14.494 | 69.00M | 14.024 | 71.31M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | nan | — |
| 1 | 5 | 0.331 | nan | — |
| 1 | 10 | 0.485 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.219 | nan | — |
| 10 | 10 | 0.465 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.222 | nan | — |
| 100 | 10 | 0.480 | nan | — |
| 1,000 | 1 | 0.062 | nan | — |
| 1,000 | 5 | 0.247 | nan | — |
| 1,000 | 10 | 0.532 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
