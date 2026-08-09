# KeltnerChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.58M | 0.014 | 72.28M | nan | — | — |
| 10,000 | 0.084 | 119.30M | 0.078 | 128.69M | nan | — | — |
| 100,000 | 0.818 | 122.25M | 0.749 | 133.51M | nan | — | — |
| 1,000,000 | 10.713 | 93.34M | 8.334 | 119.99M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | nan | — |
| 1 | 5 | 0.327 | nan | — |
| 1 | 10 | 0.480 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.225 | nan | — |
| 10 | 10 | 0.473 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.259 | nan | — |
| 100 | 10 | 0.479 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.244 | nan | — |
| 1,000 | 10 | 0.506 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
