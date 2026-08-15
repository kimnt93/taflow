# FibonacciArcs benchmark (`FibArcs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.12M | 0.016 | 61.48M | 0.495 | 27.75× | 30.40× |
| 10,000 | 0.164 | 61.06M | 0.148 | 67.59M | 4.118 | 25.14× | 27.83× |
| 100,000 | 1.629 | 61.37M | 1.517 | 65.92M | 42.090 | 25.83× | 27.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.052 | 0.204 | 3.95× |
| 1 | 5 | 0.291 | 0.896 | 3.08× |
| 1 | 10 | 0.388 | 1.810 | 4.66× |
| 10 | 1 | 0.046 | 0.170 | 3.70× |
| 10 | 5 | 0.215 | 0.884 | 4.11× |
| 10 | 10 | 0.438 | 1.894 | 4.32× |
| 100 | 1 | 0.050 | 0.250 | 4.96× |
| 100 | 5 | 0.227 | 1.068 | 4.71× |
| 100 | 10 | 0.428 | 2.246 | 5.25× |
| 1,000 | 1 | 0.080 | 0.808 | 10.12× |
| 1,000 | 5 | 0.196 | 3.075 | 15.65× |
| 1,000 | 10 | 0.503 | 6.433 | 12.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
