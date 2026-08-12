# ChaikinMoneyFlow benchmark (`ChaikinMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.55M | 0.016 | 61.80M | 0.295 | 17.25× | 18.21× |
| 10,000 | 0.077 | 129.83M | 0.073 | 137.03M | 1.552 | 20.14× | 21.26× |
| 100,000 | 0.716 | 139.63M | 0.680 | 147.14M | 14.630 | 20.43× | 21.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.408 | 3.26× |
| 1 | 5 | 0.335 | 1.250 | 3.73× |
| 1 | 10 | 0.570 | 2.932 | 5.15× |
| 10 | 1 | 0.065 | 0.233 | 3.59× |
| 10 | 5 | 0.253 | 1.159 | 4.57× |
| 10 | 10 | 0.718 | 2.961 | 4.12× |
| 100 | 1 | 0.069 | 0.260 | 3.78× |
| 100 | 5 | 0.314 | 1.676 | 5.34× |
| 100 | 10 | 0.610 | 2.858 | 4.69× |
| 1,000 | 1 | 0.068 | 0.367 | 5.38× |
| 1,000 | 5 | 0.273 | 2.323 | 8.49× |
| 1,000 | 10 | 0.653 | 4.267 | 6.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
