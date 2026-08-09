# DecayLinear benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.98M | 0.006 | 168.93M | nan | — | — |
| 10,000 | 0.039 | 253.32M | 0.036 | 278.51M | nan | — | — |
| 100,000 | 0.354 | 282.70M | 0.422 | 237.03M | nan | — | — |
| 1,000,000 | 4.624 | 216.25M | 3.926 | 254.71M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | nan | — |
| 1 | 5 | 0.293 | nan | — |
| 1 | 10 | 0.430 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.262 | nan | — |
| 10 | 10 | 0.483 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.221 | nan | — |
| 100 | 10 | 0.478 | nan | — |
| 1,000 | 1 | 0.050 | nan | — |
| 1,000 | 5 | 0.235 | nan | — |
| 1,000 | 10 | 0.494 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
