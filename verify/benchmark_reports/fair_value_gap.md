# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.70M | 0.016 | 63.34M | nan | — | — |
| 10,000 | 0.118 | 84.80M | 0.109 | 92.14M | nan | — | — |
| 100,000 | 1.220 | 81.96M | 1.035 | 96.62M | nan | — | — |
| 1,000,000 | 24.829 | 40.28M | 18.608 | 53.74M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | nan | — |
| 1 | 5 | 0.407 | nan | — |
| 1 | 10 | 0.501 | nan | — |
| 10 | 1 | 0.056 | nan | — |
| 10 | 5 | 0.240 | nan | — |
| 10 | 10 | 0.520 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.265 | nan | — |
| 100 | 10 | 0.575 | nan | — |
| 1,000 | 1 | 0.062 | nan | — |
| 1,000 | 5 | 0.259 | nan | — |
| 1,000 | 10 | 0.677 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
