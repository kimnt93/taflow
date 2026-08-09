# Ichimoku benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.093 | 10.74M | 0.096 | 10.43M | nan | — | — |
| 10,000 | 0.902 | 11.09M | 0.900 | 11.11M | nan | — | — |
| 100,000 | 9.022 | 11.08M | 8.951 | 11.17M | nan | — | — |
| 1,000,000 | 114.513 | 8.73M | 103.460 | 9.67M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | nan | — |
| 1 | 5 | 0.328 | nan | — |
| 1 | 10 | 0.515 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.232 | nan | — |
| 10 | 10 | 0.535 | nan | — |
| 100 | 1 | 0.059 | nan | — |
| 100 | 5 | 0.252 | nan | — |
| 100 | 10 | 0.538 | nan | — |
| 1,000 | 1 | 0.151 | nan | — |
| 1,000 | 5 | 0.333 | nan | — |
| 1,000 | 10 | 0.633 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
