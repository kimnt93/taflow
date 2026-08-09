# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.07M | 0.011 | 92.27M | nan | — | — |
| 10,000 | 0.081 | 123.97M | 0.074 | 135.78M | nan | — | — |
| 100,000 | 0.758 | 131.86M | 0.689 | 145.20M | nan | — | — |
| 1,000,000 | 8.462 | 118.18M | 7.375 | 135.60M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | nan | — |
| 1 | 5 | 0.285 | nan | — |
| 1 | 10 | 0.535 | nan | — |
| 10 | 1 | 0.062 | nan | — |
| 10 | 5 | 0.284 | nan | — |
| 10 | 10 | 0.564 | nan | — |
| 100 | 1 | 0.056 | nan | — |
| 100 | 5 | 0.289 | nan | — |
| 100 | 10 | 0.584 | nan | — |
| 1,000 | 1 | 0.059 | nan | — |
| 1,000 | 5 | 0.295 | nan | — |
| 1,000 | 10 | 0.586 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
