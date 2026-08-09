# VolumePriceTrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.67M | 0.006 | 176.98M | nan | — | — |
| 10,000 | 0.032 | 312.85M | 0.028 | 361.34M | nan | — | — |
| 100,000 | 0.268 | 373.33M | 0.249 | 401.11M | nan | — | — |
| 1,000,000 | 3.036 | 329.35M | 2.792 | 358.11M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | nan | — |
| 1 | 5 | 0.330 | nan | — |
| 1 | 10 | 0.476 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.210 | nan | — |
| 10 | 10 | 0.482 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.216 | nan | — |
| 100 | 10 | 0.489 | nan | — |
| 1,000 | 1 | 0.052 | nan | — |
| 1,000 | 5 | 0.239 | nan | — |
| 1,000 | 10 | 0.509 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
