# OrnsteinUhlenbeckHalfLife benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.29M | 0.047 | 21.17M | nan | — | — |
| 10,000 | 0.465 | 21.51M | 0.466 | 21.47M | nan | — | — |
| 100,000 | 4.817 | 20.76M | 4.641 | 21.55M | nan | — | — |
| 1,000,000 | 49.463 | 20.22M | 49.999 | 20.00M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | nan | — |
| 1 | 5 | 0.300 | nan | — |
| 1 | 10 | 0.543 | nan | — |
| 10 | 1 | 0.058 | nan | — |
| 10 | 5 | 0.265 | nan | — |
| 10 | 10 | 0.471 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.228 | nan | — |
| 100 | 10 | 0.545 | nan | — |
| 1,000 | 1 | 0.103 | nan | — |
| 1,000 | 5 | 0.277 | nan | — |
| 1,000 | 10 | 0.579 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
