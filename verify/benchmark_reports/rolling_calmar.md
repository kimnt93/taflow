# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.82M | 0.047 | 21.09M | nan | — | — |
| 10,000 | 0.464 | 21.55M | 0.440 | 22.73M | nan | — | — |
| 100,000 | 4.505 | 22.20M | 4.376 | 22.85M | nan | — | — |
| 1,000,000 | 45.305 | 22.07M | 43.782 | 22.84M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | nan | — |
| 1 | 5 | 0.380 | nan | — |
| 1 | 10 | 0.480 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.202 | nan | — |
| 10 | 10 | 0.457 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.207 | nan | — |
| 100 | 10 | 0.455 | nan | — |
| 1,000 | 1 | 0.097 | nan | — |
| 1,000 | 5 | 0.242 | nan | — |
| 1,000 | 10 | 0.499 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
