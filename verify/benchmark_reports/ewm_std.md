# ExponentiallyWeightedStandardDeviation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.55M | 0.007 | 149.12M | nan | — | — |
| 10,000 | 0.045 | 221.50M | 0.041 | 241.38M | nan | — | — |
| 100,000 | 0.413 | 242.26M | 0.390 | 256.52M | nan | — | — |
| 1,000,000 | 4.608 | 217.00M | 4.193 | 238.49M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | nan | — |
| 1 | 5 | 0.407 | nan | — |
| 1 | 10 | 0.465 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.225 | nan | — |
| 10 | 10 | 0.464 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.209 | nan | — |
| 100 | 10 | 0.452 | nan | — |
| 1,000 | 1 | 0.054 | nan | — |
| 1,000 | 5 | 0.212 | nan | — |
| 1,000 | 10 | 0.483 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
