# SchaffTrendCycle benchmark (`stc` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.19M | 0.055 | 18.03M | 29.214 | 502.22× | 526.63× |
| 10,000 | 0.630 | 15.88M | 0.621 | 16.09M | 284.687 | 452.20× | 458.19× |
| 100,000 | 6.227 | 16.06M | 6.157 | 16.24M | 2924.619 | 469.67× | 475.04× |
| 1,000,000 | 63.981 | 15.63M | 61.820 | 16.18M | 27951.147 | 436.87× | 452.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.238 | 2.98× |
| 1 | 5 | 0.295 | 1.021 | 3.46× |
| 1 | 10 | 0.477 | 2.000 | 4.19× |
| 10 | 1 | 0.054 | 0.197 | 3.63× |
| 10 | 5 | 0.233 | 0.995 | 4.27× |
| 10 | 10 | 0.500 | 2.239 | 4.48× |
| 100 | 1 | 0.089 | 5.131 | 57.53× |
| 100 | 5 | 0.277 | 24.182 | 87.38× |
| 100 | 10 | 0.548 | 50.806 | 92.67× |
| 1,000 | 1 | 0.178 | 28.810 | 161.58× |
| 1,000 | 5 | 0.459 | 171.300 | 373.53× |
| 1,000 | 10 | 0.616 | 367.443 | 596.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
