# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.75M | 0.014 | 73.42M | nan | — | — |
| 10,000 | 0.113 | 88.46M | 0.101 | 98.71M | nan | — | — |
| 100,000 | 1.007 | 99.35M | 0.964 | 103.70M | nan | — | — |
| 1,000,000 | 11.222 | 89.11M | 10.046 | 99.54M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | nan | — |
| 1 | 5 | 0.264 | nan | — |
| 1 | 10 | 0.452 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.210 | nan | — |
| 10 | 10 | 0.463 | nan | — |
| 100 | 1 | 0.055 | nan | — |
| 100 | 5 | 0.238 | nan | — |
| 100 | 10 | 0.445 | nan | — |
| 1,000 | 1 | 0.062 | nan | — |
| 1,000 | 5 | 0.248 | nan | — |
| 1,000 | 10 | 0.556 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
