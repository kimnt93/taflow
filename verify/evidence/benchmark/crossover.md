# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 374.66M | 0.001 | 759.45M | 0.017 | 6.46× | 13.09× |
| 10,000 | 0.009 | 1.10G | 0.005 | 1.85G | 0.031 | 3.40× | 5.72× |
| 100,000 | 0.074 | 1.34G | 0.049 | 2.06G | 0.136 | 1.82× | 2.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.093 | 0.92× |
| 1 | 5 | 0.281 | 0.369 | 1.31× |
| 1 | 10 | 0.398 | 0.674 | 1.69× |
| 10 | 1 | 0.040 | 0.068 | 1.71× |
| 10 | 5 | 0.174 | 0.311 | 1.78× |
| 10 | 10 | 0.412 | 0.745 | 1.81× |
| 100 | 1 | 0.047 | 0.082 | 1.74× |
| 100 | 5 | 0.191 | 0.311 | 1.63× |
| 100 | 10 | 0.389 | 0.669 | 1.72× |
| 1,000 | 1 | 0.040 | 0.069 | 1.72× |
| 1,000 | 5 | 0.184 | 0.398 | 2.17× |
| 1,000 | 10 | 0.381 | 1.023 | 2.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
