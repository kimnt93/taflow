# CrabPattern benchmark (`Crab` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.32M | 0.012 | 83.94M | 0.225 | 16.06× | 18.90× |
| 10,000 | 0.096 | 104.65M | 0.091 | 109.55M | 1.464 | 15.32× | 16.03× |
| 100,000 | 0.906 | 110.36M | 1.183 | 84.54M | 13.575 | 14.98× | 11.48× |
| 1,000,000 | 10.615 | 94.21M | 9.776 | 102.29M | 131.634 | 12.40× | 13.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.190 | 2.08× |
| 1 | 5 | 0.345 | 1.140 | 3.31× |
| 1 | 10 | 0.533 | 1.646 | 3.09× |
| 10 | 1 | 0.056 | 0.171 | 3.06× |
| 10 | 5 | 0.248 | 1.092 | 4.40× |
| 10 | 10 | 0.554 | 1.655 | 2.99× |
| 100 | 1 | 0.056 | 0.182 | 3.26× |
| 100 | 5 | 0.264 | 1.124 | 4.26× |
| 100 | 10 | 0.548 | 1.796 | 3.28× |
| 1,000 | 1 | 0.065 | 0.307 | 4.72× |
| 1,000 | 5 | 0.257 | 1.717 | 6.69× |
| 1,000 | 10 | 0.567 | 3.002 | 5.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
