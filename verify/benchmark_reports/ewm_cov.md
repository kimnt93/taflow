# ExponentiallyWeightedCovariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.50M | 0.008 | 131.71M | nan | — | — |
| 10,000 | 0.052 | 192.37M | 0.049 | 205.36M | nan | — | — |
| 100,000 | 0.514 | 194.43M | 0.483 | 206.86M | nan | — | — |
| 1,000,000 | 5.614 | 178.14M | 4.729 | 211.46M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | nan | — |
| 1 | 5 | 0.274 | nan | — |
| 1 | 10 | 0.502 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.228 | nan | — |
| 10 | 10 | 0.483 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.228 | nan | — |
| 100 | 10 | 0.490 | nan | — |
| 1,000 | 1 | 0.051 | nan | — |
| 1,000 | 5 | 0.243 | nan | — |
| 1,000 | 10 | 0.526 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
