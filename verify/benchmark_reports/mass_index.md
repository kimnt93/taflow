# MassIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.01M | 0.009 | 115.89M | nan | — | — |
| 10,000 | 0.061 | 165.08M | 0.057 | 174.33M | nan | — | — |
| 100,000 | 0.617 | 162.02M | 0.538 | 185.78M | nan | — | — |
| 1,000,000 | 5.952 | 168.00M | 5.449 | 183.52M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | nan | — |
| 1 | 5 | 0.438 | nan | — |
| 1 | 10 | 0.457 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.209 | nan | — |
| 10 | 10 | 0.467 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.227 | nan | — |
| 100 | 10 | 0.481 | nan | — |
| 1,000 | 1 | 0.056 | nan | — |
| 1,000 | 5 | 0.235 | nan | — |
| 1,000 | 10 | 0.510 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
