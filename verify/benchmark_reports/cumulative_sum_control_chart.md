# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.82M | 0.006 | 168.49M | nan | — | — |
| 10,000 | 0.040 | 252.71M | 0.040 | 248.77M | nan | — | — |
| 100,000 | 0.375 | 266.72M | 0.341 | 293.12M | nan | — | — |
| 1,000,000 | 4.093 | 244.33M | 3.578 | 279.46M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | nan | — |
| 1 | 5 | 0.292 | nan | — |
| 1 | 10 | 0.488 | nan | — |
| 10 | 1 | 0.045 | nan | — |
| 10 | 5 | 0.203 | nan | — |
| 10 | 10 | 0.474 | nan | — |
| 100 | 1 | 0.055 | nan | — |
| 100 | 5 | 0.238 | nan | — |
| 100 | 10 | 0.515 | nan | — |
| 1,000 | 1 | 0.053 | nan | — |
| 1,000 | 5 | 0.230 | nan | — |
| 1,000 | 10 | 0.471 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
