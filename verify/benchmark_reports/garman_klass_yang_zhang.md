# GarmanKlassYangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.11M | 0.029 | 34.30M | nan | — | — |
| 10,000 | 0.232 | 43.05M | 0.242 | 41.33M | nan | — | — |
| 100,000 | 2.191 | 45.64M | 2.373 | 42.15M | nan | — | — |
| 1,000,000 | 26.290 | 38.04M | 23.117 | 43.26M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | nan | — |
| 1 | 5 | 0.423 | nan | — |
| 1 | 10 | 0.566 | nan | — |
| 10 | 1 | 0.057 | nan | — |
| 10 | 5 | 0.246 | nan | — |
| 10 | 10 | 0.542 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.254 | nan | — |
| 100 | 10 | 0.560 | nan | — |
| 1,000 | 1 | 0.083 | nan | — |
| 1,000 | 5 | 0.304 | nan | — |
| 1,000 | 10 | 0.603 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
