# ForceIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.79M | 0.006 | 170.09M | nan | — | — |
| 10,000 | 0.032 | 314.58M | 0.029 | 343.95M | nan | — | — |
| 100,000 | 0.280 | 357.40M | 0.258 | 387.01M | nan | — | — |
| 1,000,000 | 3.315 | 301.62M | 2.765 | 361.61M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | nan | — |
| 1 | 5 | 0.301 | nan | — |
| 1 | 10 | 0.481 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.235 | nan | — |
| 10 | 10 | 0.501 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.216 | nan | — |
| 100 | 10 | 0.487 | nan | — |
| 1,000 | 1 | 0.051 | nan | — |
| 1,000 | 5 | 0.227 | nan | — |
| 1,000 | 10 | 0.488 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
