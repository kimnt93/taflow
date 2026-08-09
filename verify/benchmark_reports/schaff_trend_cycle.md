# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.11M | 0.057 | 17.54M | nan | — | — |
| 10,000 | 0.610 | 16.39M | 0.609 | 16.43M | nan | — | — |
| 100,000 | 6.191 | 16.15M | 6.028 | 16.59M | nan | — | — |
| 1,000,000 | 71.054 | 14.07M | 63.345 | 15.79M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | nan | — |
| 1 | 5 | 0.378 | nan | — |
| 1 | 10 | 0.627 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.230 | nan | — |
| 10 | 10 | 0.463 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.245 | nan | — |
| 100 | 10 | 0.506 | nan | — |
| 1,000 | 1 | 0.117 | nan | — |
| 1,000 | 5 | 0.254 | nan | — |
| 1,000 | 10 | 0.560 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
