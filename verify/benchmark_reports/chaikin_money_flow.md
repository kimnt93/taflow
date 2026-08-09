# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.18M | 0.011 | 88.66M | nan | — | — |
| 10,000 | 0.069 | 144.30M | 0.065 | 153.00M | nan | — | — |
| 100,000 | 0.610 | 163.90M | 0.629 | 159.07M | nan | — | — |
| 1,000,000 | 6.506 | 153.71M | 6.052 | 165.24M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | nan | — |
| 1 | 5 | 0.263 | nan | — |
| 1 | 10 | 0.521 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.255 | nan | — |
| 10 | 10 | 0.509 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.245 | nan | — |
| 100 | 10 | 0.543 | nan | — |
| 1,000 | 1 | 0.058 | nan | — |
| 1,000 | 5 | 0.250 | nan | — |
| 1,000 | 10 | 0.541 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
