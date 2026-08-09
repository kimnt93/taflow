# DonchianChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.59M | 0.008 | 118.13M | nan | — | — |
| 10,000 | 0.084 | 118.80M | 0.082 | 122.51M | nan | — | — |
| 100,000 | 0.868 | 115.25M | 0.744 | 134.44M | nan | — | — |
| 1,000,000 | 19.521 | 51.23M | 8.891 | 112.47M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | nan | — |
| 1 | 5 | 0.487 | nan | — |
| 1 | 10 | 0.491 | nan | — |
| 10 | 1 | 0.052 | nan | — |
| 10 | 5 | 0.214 | nan | — |
| 10 | 10 | 0.470 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.218 | nan | — |
| 100 | 10 | 0.475 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.235 | nan | — |
| 1,000 | 10 | 0.492 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
