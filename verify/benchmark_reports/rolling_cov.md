# RollingCov benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.69M | 0.035 | 28.94M | nan | — | — |
| 10,000 | 0.338 | 29.61M | 0.322 | 31.08M | nan | — | — |
| 100,000 | 3.130 | 31.95M | 3.175 | 31.50M | nan | — | — |
| 1,000,000 | 31.590 | 31.66M | 31.416 | 31.83M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | nan | — |
| 1 | 5 | 0.406 | nan | — |
| 1 | 10 | 0.483 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.211 | nan | — |
| 10 | 10 | 0.475 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.232 | nan | — |
| 100 | 10 | 0.525 | nan | — |
| 1,000 | 1 | 0.080 | nan | — |
| 1,000 | 5 | 0.235 | nan | — |
| 1,000 | 10 | 0.509 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
