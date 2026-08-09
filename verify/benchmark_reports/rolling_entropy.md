# RollingEntropy benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.688 | 1.45M | 0.709 | 1.41M | nan | — | — |
| 10,000 | 6.771 | 1.48M | 7.395 | 1.35M | nan | — | — |
| 100,000 | 71.922 | 1.39M | 71.241 | 1.40M | nan | — | — |
| 1,000,000 | 680.448 | 1.47M | 682.312 | 1.47M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | nan | — |
| 1 | 5 | 0.399 | nan | — |
| 1 | 10 | 0.567 | nan | — |
| 10 | 1 | 0.052 | nan | — |
| 10 | 5 | 0.215 | nan | — |
| 10 | 10 | 0.477 | nan | — |
| 100 | 1 | 0.112 | nan | — |
| 100 | 5 | 0.251 | nan | — |
| 100 | 10 | 0.507 | nan | — |
| 1,000 | 1 | 0.792 | nan | — |
| 1,000 | 5 | 1.073 | nan | — |
| 1,000 | 10 | 1.521 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
