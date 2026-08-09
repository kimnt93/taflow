# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.10M | 0.007 | 143.42M | nan | — | — |
| 10,000 | 0.048 | 207.55M | 0.045 | 220.60M | nan | — | — |
| 100,000 | 0.458 | 218.47M | 0.427 | 233.92M | nan | — | — |
| 1,000,000 | 5.035 | 198.63M | 4.564 | 219.13M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | nan | — |
| 1 | 5 | 0.281 | nan | — |
| 1 | 10 | 0.510 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.231 | nan | — |
| 10 | 10 | 0.480 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.247 | nan | — |
| 100 | 10 | 0.574 | nan | — |
| 1,000 | 1 | 0.055 | nan | — |
| 1,000 | 5 | 0.277 | nan | — |
| 1,000 | 10 | 0.584 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
