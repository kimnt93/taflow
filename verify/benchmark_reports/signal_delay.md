# SignalDelay benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.07M | 0.006 | 177.87M | nan | — | — |
| 10,000 | 0.038 | 260.54M | 0.035 | 287.39M | nan | — | — |
| 100,000 | 0.373 | 268.13M | 0.324 | 308.47M | nan | — | — |
| 1,000,000 | 3.779 | 264.64M | 3.254 | 307.30M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | nan | — |
| 1 | 5 | 0.331 | nan | — |
| 1 | 10 | 0.458 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.221 | nan | — |
| 10 | 10 | 0.461 | nan | — |
| 100 | 1 | 0.044 | nan | — |
| 100 | 5 | 0.210 | nan | — |
| 100 | 10 | 0.457 | nan | — |
| 1,000 | 1 | 0.055 | nan | — |
| 1,000 | 5 | 0.229 | nan | — |
| 1,000 | 10 | 0.481 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
