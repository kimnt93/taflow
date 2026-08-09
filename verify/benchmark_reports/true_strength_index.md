# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.59M | 0.008 | 132.78M | nan | — | — |
| 10,000 | 0.059 | 170.11M | 0.056 | 179.25M | nan | — | — |
| 100,000 | 0.539 | 185.38M | 0.513 | 194.85M | nan | — | — |
| 1,000,000 | 5.647 | 177.09M | 5.104 | 195.91M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | nan | — |
| 1 | 5 | 0.352 | nan | — |
| 1 | 10 | 0.452 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.211 | nan | — |
| 10 | 10 | 0.423 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.226 | nan | — |
| 100 | 10 | 0.448 | nan | — |
| 1,000 | 1 | 0.054 | nan | — |
| 1,000 | 5 | 0.224 | nan | — |
| 1,000 | 10 | 0.485 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
