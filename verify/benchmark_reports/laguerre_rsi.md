# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.92M | 0.011 | 91.68M | nan | — | — |
| 10,000 | 0.078 | 127.61M | 0.078 | 128.84M | nan | — | — |
| 100,000 | 0.810 | 123.41M | 0.757 | 132.15M | nan | — | — |
| 1,000,000 | 8.200 | 121.96M | 7.673 | 130.33M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | nan | — |
| 1 | 5 | 0.329 | nan | — |
| 1 | 10 | 0.449 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.219 | nan | — |
| 10 | 10 | 0.422 | nan | — |
| 100 | 1 | 0.046 | nan | — |
| 100 | 5 | 0.207 | nan | — |
| 100 | 10 | 0.425 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.237 | nan | — |
| 1,000 | 10 | 0.513 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
