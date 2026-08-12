# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.85M | 0.017 | 59.08M | 0.040 | 2.00× | 2.37× |
| 10,000 | 0.141 | 70.88M | 0.136 | 73.30M | 0.112 | 0.79× | 0.82× |
| 100,000 | 1.389 | 71.99M | 1.436 | 69.65M | 0.852 | 0.61× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.118 | 0.74× |
| 1 | 5 | 0.446 | 0.502 | 1.12× |
| 1 | 10 | 0.543 | 0.984 | 1.81× |
| 10 | 1 | 0.060 | 0.091 | 1.51× |
| 10 | 5 | 0.272 | 0.464 | 1.71× |
| 10 | 10 | 0.564 | 1.006 | 1.78× |
| 100 | 1 | 0.058 | 0.099 | 1.69× |
| 100 | 5 | 0.261 | 0.456 | 1.75× |
| 100 | 10 | 0.516 | 0.976 | 1.89× |
| 1,000 | 1 | 0.069 | 0.106 | 1.53× |
| 1,000 | 5 | 0.304 | 0.565 | 1.86× |
| 1,000 | 10 | 0.578 | 1.025 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
