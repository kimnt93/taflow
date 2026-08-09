# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.67M | 0.007 | 143.46M | nan | — | — |
| 10,000 | 0.048 | 208.66M | 0.045 | 222.02M | nan | — | — |
| 100,000 | 0.512 | 195.30M | 0.410 | 243.93M | nan | — | — |
| 1,000,000 | 4.904 | 203.93M | 4.301 | 232.52M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | nan | — |
| 1 | 5 | 0.247 | nan | — |
| 1 | 10 | 0.547 | nan | — |
| 10 | 1 | 0.061 | nan | — |
| 10 | 5 | 0.220 | nan | — |
| 10 | 10 | 0.470 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.213 | nan | — |
| 100 | 10 | 0.443 | nan | — |
| 1,000 | 1 | 0.050 | nan | — |
| 1,000 | 5 | 0.253 | nan | — |
| 1,000 | 10 | 0.504 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
