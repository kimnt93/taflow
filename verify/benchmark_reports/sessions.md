# Sessions benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.34M | 0.012 | 81.02M | nan | — | — |
| 10,000 | 0.084 | 118.91M | 0.074 | 135.86M | nan | — | — |
| 100,000 | 0.830 | 120.53M | 0.696 | 143.69M | nan | — | — |
| 1,000,000 | 18.479 | 54.11M | 8.683 | 115.16M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | nan | — |
| 1 | 5 | 0.478 | nan | — |
| 1 | 10 | 0.497 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.219 | nan | — |
| 10 | 10 | 0.461 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.238 | nan | — |
| 100 | 10 | 0.493 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.250 | nan | — |
| 1,000 | 10 | 0.513 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
