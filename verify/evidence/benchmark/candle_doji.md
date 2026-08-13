# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.063 | 15.79M | 0.052 | 19.17M | 0.031 | 0.48× | 0.59× |
| 10,000 | 0.442 | 22.64M | 0.407 | 24.59M | 0.050 | 0.11× | 0.12× |
| 100,000 | 4.199 | 23.82M | 3.954 | 25.29M | 0.249 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.124 | 1.04× |
| 1 | 5 | 0.389 | 0.489 | 1.26× |
| 1 | 10 | 0.633 | 0.905 | 1.43× |
| 10 | 1 | 0.066 | 0.091 | 1.39× |
| 10 | 5 | 0.306 | 0.433 | 1.41× |
| 10 | 10 | 0.636 | 0.916 | 1.44× |
| 100 | 1 | 0.069 | 0.088 | 1.28× |
| 100 | 5 | 0.314 | 0.423 | 1.34× |
| 100 | 10 | 0.659 | 0.907 | 1.38× |
| 1,000 | 1 | 0.107 | 0.089 | 0.83× |
| 1,000 | 5 | 0.320 | 0.470 | 1.47× |
| 1,000 | 10 | 0.732 | 1.004 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
