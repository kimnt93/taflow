# OrderBlock benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.92M | 0.082 | 12.17M | nan | — | — |
| 10,000 | 0.878 | 11.39M | 0.892 | 11.21M | nan | — | — |
| 100,000 | 9.330 | 10.72M | 9.004 | 11.11M | nan | — | — |
| 1,000,000 | 109.978 | 9.09M | 101.415 | 9.86M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | nan | — |
| 1 | 5 | 0.327 | nan | — |
| 1 | 10 | 0.556 | nan | — |
| 10 | 1 | 0.094 | nan | — |
| 10 | 5 | 0.746 | nan | — |
| 10 | 10 | 0.646 | nan | — |
| 100 | 1 | 0.067 | nan | — |
| 100 | 5 | 0.274 | nan | — |
| 100 | 10 | 0.544 | nan | — |
| 1,000 | 1 | 0.144 | nan | — |
| 1,000 | 5 | 0.306 | nan | — |
| 1,000 | 10 | 0.690 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
