# AwesomeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.61M | 0.026 | 38.50M | nan | — | — |
| 10,000 | 0.230 | 43.44M | 0.233 | 42.83M | nan | — | — |
| 100,000 | 2.316 | 43.18M | 2.395 | 41.75M | nan | — | — |
| 1,000,000 | 23.875 | 41.88M | 24.374 | 41.03M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | nan | — |
| 1 | 5 | 0.417 | nan | — |
| 1 | 10 | 0.466 | nan | — |
| 10 | 1 | 0.044 | nan | — |
| 10 | 5 | 0.220 | nan | — |
| 10 | 10 | 0.460 | nan | — |
| 100 | 1 | 0.046 | nan | — |
| 100 | 5 | 0.228 | nan | — |
| 100 | 10 | 0.452 | nan | — |
| 1,000 | 1 | 0.077 | nan | — |
| 1,000 | 5 | 0.236 | nan | — |
| 1,000 | 10 | 0.516 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
