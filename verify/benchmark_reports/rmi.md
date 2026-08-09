# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.42M | 0.008 | 126.30M | nan | — | — |
| 10,000 | 0.067 | 148.97M | 0.064 | 157.09M | nan | — | — |
| 100,000 | 0.643 | 155.47M | 0.625 | 160.09M | nan | — | — |
| 1,000,000 | 6.595 | 151.64M | 6.380 | 156.73M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.199 | nan | — |
| 1 | 5 | 0.286 | nan | — |
| 1 | 10 | 0.482 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.202 | nan | — |
| 10 | 10 | 0.459 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.219 | nan | — |
| 100 | 10 | 0.541 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.220 | nan | — |
| 1,000 | 10 | 0.481 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
