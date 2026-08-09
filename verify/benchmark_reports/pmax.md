# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.70M | 0.022 | 44.63M | nan | — | — |
| 10,000 | 0.192 | 51.98M | 0.186 | 53.82M | nan | — | — |
| 100,000 | 1.927 | 51.90M | 1.900 | 52.62M | nan | — | — |
| 1,000,000 | 19.906 | 50.24M | 18.860 | 53.02M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | nan | — |
| 1 | 5 | 0.270 | nan | — |
| 1 | 10 | 0.458 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.200 | nan | — |
| 10 | 10 | 0.403 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.228 | nan | — |
| 100 | 10 | 0.431 | nan | — |
| 1,000 | 1 | 0.077 | nan | — |
| 1,000 | 5 | 0.318 | nan | — |
| 1,000 | 10 | 0.670 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
