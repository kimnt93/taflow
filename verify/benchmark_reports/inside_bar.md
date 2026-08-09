# InsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.29M | 0.006 | 175.86M | nan | — | — |
| 10,000 | 0.032 | 308.47M | 0.029 | 345.45M | nan | — | — |
| 100,000 | 0.304 | 328.45M | 0.261 | 382.57M | nan | — | — |
| 1,000,000 | 3.268 | 305.99M | 2.870 | 348.49M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | nan | — |
| 1 | 5 | 0.305 | nan | — |
| 1 | 10 | 0.556 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.271 | nan | — |
| 10 | 10 | 0.565 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.287 | nan | — |
| 100 | 10 | 0.581 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.284 | nan | — |
| 1,000 | 10 | 0.586 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
