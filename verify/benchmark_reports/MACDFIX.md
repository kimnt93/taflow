# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.66M | 0.005 | 186.96M | 0.053 | 8.00× | 9.87× |
| 10,000 | 0.037 | 271.53M | 0.028 | 355.12M | 0.140 | 3.80× | 4.96× |
| 100,000 | 0.328 | 304.52M | 0.246 | 406.59M | 1.009 | 3.07× | 4.10× |
| 1,000,000 | 13.276 | 75.32M | 2.734 | 365.72M | 11.830 | 0.89× | 4.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.130 | 1.82× |
| 1 | 5 | 0.320 | 0.589 | 1.84× |
| 1 | 10 | 0.532 | 1.300 | 2.44× |
| 10 | 1 | 0.053 | 0.111 | 2.10× |
| 10 | 5 | 0.258 | 0.513 | 1.99× |
| 10 | 10 | 0.526 | 1.184 | 2.25× |
| 100 | 1 | 0.051 | 0.108 | 2.12× |
| 100 | 5 | 0.233 | 0.532 | 2.28× |
| 100 | 10 | 0.519 | 1.168 | 2.25× |
| 1,000 | 1 | 0.064 | 0.118 | 1.86× |
| 1,000 | 5 | 0.236 | 0.590 | 2.50× |
| 1,000 | 10 | 0.515 | 1.240 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
