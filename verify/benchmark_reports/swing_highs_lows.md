# SwingHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.30M | 0.045 | 22.45M | nan | — | — |
| 10,000 | 0.379 | 26.40M | 0.364 | 27.48M | nan | — | — |
| 100,000 | 3.678 | 27.19M | 3.666 | 27.28M | nan | — | — |
| 1,000,000 | 47.343 | 21.12M | 36.505 | 27.39M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | nan | — |
| 1 | 5 | 0.299 | nan | — |
| 1 | 10 | 0.558 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.241 | nan | — |
| 10 | 10 | 0.504 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.250 | nan | — |
| 100 | 10 | 0.514 | nan | — |
| 1,000 | 1 | 0.092 | nan | — |
| 1,000 | 5 | 0.242 | nan | — |
| 1,000 | 10 | 0.545 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
