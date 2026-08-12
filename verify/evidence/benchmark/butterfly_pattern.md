# ButterflyPattern benchmark (`Butterfly` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.42M | 0.012 | 82.97M | 0.227 | 14.85× | 18.84× |
| 10,000 | 0.097 | 103.11M | 0.100 | 99.53M | 1.387 | 14.30× | 13.80× |
| 100,000 | 0.935 | 107.00M | 0.904 | 110.61M | 17.398 | 18.62× | 19.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.245 | 2.49× |
| 1 | 5 | 0.375 | 0.838 | 2.24× |
| 1 | 10 | 0.547 | 1.876 | 3.43× |
| 10 | 1 | 0.057 | 0.177 | 3.10× |
| 10 | 5 | 0.271 | 1.117 | 4.13× |
| 10 | 10 | 0.613 | 1.888 | 3.08× |
| 100 | 1 | 0.065 | 0.184 | 2.85× |
| 100 | 5 | 0.265 | 1.220 | 4.60× |
| 100 | 10 | 0.645 | 1.855 | 2.88× |
| 1,000 | 1 | 0.074 | 0.299 | 4.06× |
| 1,000 | 5 | 0.273 | 1.848 | 6.77× |
| 1,000 | 10 | 0.593 | 3.109 | 5.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
