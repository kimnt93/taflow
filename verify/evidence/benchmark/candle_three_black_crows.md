# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.97M | 0.011 | 94.22M | 0.035 | 2.22× | 3.32× |
| 10,000 | 0.066 | 150.49M | 0.061 | 164.69M | 0.090 | 1.36× | 1.49× |
| 100,000 | 0.813 | 123.00M | 0.806 | 124.07M | 0.694 | 0.85× | 0.86× |
| 1,000,000 | 8.304 | 120.43M | 8.110 | 123.31M | 6.947 | 0.84× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.107 | 0.87× |
| 1 | 5 | 0.321 | 0.500 | 1.56× |
| 1 | 10 | 0.633 | 2.365 | 3.74× |
| 10 | 1 | 0.089 | 0.114 | 1.28× |
| 10 | 5 | 1.674 | 0.838 | 0.50× |
| 10 | 10 | 1.483 | 3.068 | 2.07× |
| 100 | 1 | 0.086 | 0.120 | 1.39× |
| 100 | 5 | 0.648 | 1.414 | 2.18× |
| 100 | 10 | 2.679 | 3.601 | 1.34× |
| 1,000 | 1 | 0.070 | 0.138 | 1.98× |
| 1,000 | 5 | 0.408 | 0.613 | 1.50× |
| 1,000 | 10 | 0.647 | 1.101 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
