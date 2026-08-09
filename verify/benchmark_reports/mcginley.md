# McGinleyDynamic benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.48M | 0.015 | 67.96M | nan | — | — |
| 10,000 | 0.125 | 79.85M | 0.130 | 76.77M | nan | — | — |
| 100,000 | 1.277 | 78.34M | 1.203 | 83.11M | nan | — | — |
| 1,000,000 | 12.361 | 80.90M | 12.852 | 77.81M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | nan | — |
| 1 | 5 | 0.377 | nan | — |
| 1 | 10 | 0.462 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.230 | nan | — |
| 10 | 10 | 0.460 | nan | — |
| 100 | 1 | 0.046 | nan | — |
| 100 | 5 | 0.221 | nan | — |
| 100 | 10 | 0.466 | nan | — |
| 1,000 | 1 | 0.064 | nan | — |
| 1,000 | 5 | 0.219 | nan | — |
| 1,000 | 10 | 0.479 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
