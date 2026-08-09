# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.32M | 0.010 | 97.18M | nan | — | — |
| 10,000 | 0.113 | 88.33M | 0.118 | 84.70M | nan | — | — |
| 100,000 | 1.126 | 88.77M | 1.141 | 87.65M | nan | — | — |
| 1,000,000 | 11.650 | 85.84M | 12.184 | 82.07M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | nan | — |
| 1 | 5 | 0.306 | nan | — |
| 1 | 10 | 0.481 | nan | — |
| 10 | 1 | 0.045 | nan | — |
| 10 | 5 | 0.218 | nan | — |
| 10 | 10 | 0.430 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.204 | nan | — |
| 100 | 10 | 0.441 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.231 | nan | — |
| 1,000 | 10 | 0.479 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
