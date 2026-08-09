# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.090 | 11.12M | 0.089 | 11.30M | nan | — | — |
| 10,000 | 0.887 | 11.28M | 0.877 | 11.40M | nan | — | — |
| 100,000 | 8.376 | 11.94M | 8.500 | 11.76M | nan | — | — |
| 1,000,000 | 88.103 | 11.35M | 84.571 | 11.82M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | nan | — |
| 1 | 5 | 0.269 | nan | — |
| 1 | 10 | 0.446 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.198 | nan | — |
| 10 | 10 | 0.402 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.239 | nan | — |
| 100 | 10 | 0.518 | nan | — |
| 1,000 | 1 | 0.142 | nan | — |
| 1,000 | 5 | 0.668 | nan | — |
| 1,000 | 10 | 1.333 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
