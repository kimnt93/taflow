# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.10M | 0.021 | 48.71M | nan | — | — |
| 10,000 | 0.195 | 51.29M | 0.203 | 49.36M | nan | — | — |
| 100,000 | 1.916 | 52.19M | 1.966 | 50.87M | nan | — | — |
| 1,000,000 | 20.622 | 48.49M | 20.146 | 49.64M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | nan | — |
| 1 | 5 | 0.269 | nan | — |
| 1 | 10 | 0.452 | nan | — |
| 10 | 1 | 0.045 | nan | — |
| 10 | 5 | 0.192 | nan | — |
| 10 | 10 | 0.392 | nan | — |
| 100 | 1 | 0.045 | nan | — |
| 100 | 5 | 0.200 | nan | — |
| 100 | 10 | 0.419 | nan | — |
| 1,000 | 1 | 0.076 | nan | — |
| 1,000 | 5 | 0.303 | nan | — |
| 1,000 | 10 | 0.613 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
