# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.87M | 0.059 | 16.86M | nan | — | — |
| 10,000 | 0.506 | 19.77M | 0.486 | 20.57M | nan | — | — |
| 100,000 | 5.193 | 19.26M | 5.432 | 18.41M | nan | — | — |
| 1,000,000 | 50.753 | 19.70M | 49.362 | 20.26M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | nan | — |
| 1 | 5 | 0.308 | nan | — |
| 1 | 10 | 0.535 | nan | — |
| 10 | 1 | 0.057 | nan | — |
| 10 | 5 | 0.213 | nan | — |
| 10 | 10 | 0.470 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.262 | nan | — |
| 100 | 10 | 0.569 | nan | — |
| 1,000 | 1 | 0.106 | nan | — |
| 1,000 | 5 | 0.527 | nan | — |
| 1,000 | 10 | 1.049 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
