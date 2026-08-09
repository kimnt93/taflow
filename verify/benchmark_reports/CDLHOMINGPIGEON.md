# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.13M | 0.008 | 131.08M | 0.032 | 3.58× | 4.26× |
| 10,000 | 0.059 | 169.11M | 0.053 | 188.94M | 0.094 | 1.59× | 1.77× |
| 100,000 | 0.651 | 153.64M | 0.633 | 158.00M | 0.741 | 1.14× | 1.17× |
| 1,000,000 | 6.859 | 145.78M | 6.581 | 151.96M | 7.526 | 1.10× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.099 | 1.29× |
| 1 | 5 | 0.381 | 0.522 | 1.37× |
| 1 | 10 | 0.504 | 0.921 | 1.83× |
| 10 | 1 | 0.052 | 0.093 | 1.77× |
| 10 | 5 | 0.229 | 0.426 | 1.86× |
| 10 | 10 | 0.516 | 0.933 | 1.81× |
| 100 | 1 | 0.053 | 0.095 | 1.79× |
| 100 | 5 | 0.258 | 0.441 | 1.71× |
| 100 | 10 | 0.516 | 0.917 | 1.78× |
| 1,000 | 1 | 0.066 | 0.103 | 1.57× |
| 1,000 | 5 | 0.258 | 0.481 | 1.86× |
| 1,000 | 10 | 0.560 | 1.012 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
