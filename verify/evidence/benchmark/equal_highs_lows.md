# EqualHighsLows benchmark (`causal equal pivot levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.66M | 0.043 | 23.22M | 4.216 | 91.31× | 97.87× |
| 10,000 | 0.441 | 22.67M | 0.445 | 22.46M | 43.173 | 97.87× | 96.96× |
| 100,000 | 4.881 | 20.49M | 4.330 | 23.10M | 450.310 | 92.26× | 104.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.178 | 2.60× |
| 1 | 5 | 0.318 | 0.754 | 2.37× |
| 1 | 10 | 0.525 | 1.616 | 3.08× |
| 10 | 1 | 0.055 | 0.182 | 3.31× |
| 10 | 5 | 0.285 | 0.924 | 3.25× |
| 10 | 10 | 0.574 | 1.918 | 3.34× |
| 100 | 1 | 0.061 | 0.542 | 8.89× |
| 100 | 5 | 0.247 | 2.884 | 11.65× |
| 100 | 10 | 0.581 | 5.608 | 9.65× |
| 1,000 | 1 | 0.108 | 4.564 | 42.35× |
| 1,000 | 5 | 0.278 | 23.713 | 85.35× |
| 1,000 | 10 | 0.631 | 51.492 | 81.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
