# GarmanKlass benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.49M | 0.023 | 42.82M | nan | — | — |
| 10,000 | 0.182 | 54.82M | 0.180 | 55.68M | nan | — | — |
| 100,000 | 1.741 | 57.44M | 1.720 | 58.14M | nan | — | — |
| 1,000,000 | 17.991 | 55.58M | 17.040 | 58.68M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | nan | — |
| 1 | 5 | 0.323 | nan | — |
| 1 | 10 | 0.554 | nan | — |
| 10 | 1 | 0.058 | nan | — |
| 10 | 5 | 0.304 | nan | — |
| 10 | 10 | 0.623 | nan | — |
| 100 | 1 | 0.058 | nan | — |
| 100 | 5 | 0.257 | nan | — |
| 100 | 10 | 0.939 | nan | — |
| 1,000 | 1 | 0.072 | nan | — |
| 1,000 | 5 | 0.304 | nan | — |
| 1,000 | 10 | 0.642 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
