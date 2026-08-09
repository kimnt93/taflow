# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.48M | 0.005 | 221.85M | 0.031 | 5.40× | 6.87× |
| 10,000 | 0.014 | 722.79M | 0.009 | 1.08G | 0.035 | 2.54× | 3.81× |
| 100,000 | 0.079 | 1.26G | 0.053 | 1.90G | 0.083 | 1.04× | 1.57× |
| 1,000,000 | 1.664 | 601.08M | 1.051 | 951.20M | 1.106 | 0.67× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.161 | 1.68× |
| 1 | 5 | 0.352 | 0.555 | 1.58× |
| 1 | 10 | 0.490 | 0.934 | 1.90× |
| 10 | 1 | 0.051 | 0.091 | 1.80× |
| 10 | 5 | 0.227 | 0.425 | 1.87× |
| 10 | 10 | 0.493 | 0.915 | 1.85× |
| 100 | 1 | 0.050 | 0.097 | 1.95× |
| 100 | 5 | 0.230 | 0.428 | 1.86× |
| 100 | 10 | 0.510 | 0.939 | 1.84× |
| 1,000 | 1 | 0.052 | 0.097 | 1.85× |
| 1,000 | 5 | 0.254 | 0.431 | 1.69× |
| 1,000 | 10 | 0.503 | 0.956 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
