# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.45M | 0.018 | 55.82M | 0.036 | 1.69× | 2.03× |
| 10,000 | 0.138 | 72.50M | 0.133 | 75.20M | 0.112 | 0.81× | 0.84× |
| 100,000 | 1.308 | 76.46M | 1.219 | 82.01M | 0.829 | 0.63× | 0.68× |
| 1,000,000 | 12.989 | 76.99M | 12.748 | 78.44M | 8.370 | 0.64× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.148 | 1.55× |
| 1 | 5 | 0.337 | 0.452 | 1.34× |
| 1 | 10 | 0.589 | 1.044 | 1.77× |
| 10 | 1 | 0.057 | 0.102 | 1.79× |
| 10 | 5 | 0.265 | 0.460 | 1.74× |
| 10 | 10 | 0.548 | 0.927 | 1.69× |
| 100 | 1 | 0.057 | 0.105 | 1.84× |
| 100 | 5 | 0.294 | 0.465 | 1.58× |
| 100 | 10 | 0.577 | 0.946 | 1.64× |
| 1,000 | 1 | 0.077 | 0.104 | 1.34× |
| 1,000 | 5 | 0.285 | 0.491 | 1.72× |
| 1,000 | 10 | 0.565 | 0.991 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
