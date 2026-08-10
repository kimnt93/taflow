# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.14M | 0.013 | 75.99M | 0.056 | 3.46× | 4.23× |
| 10,000 | 0.096 | 103.68M | 0.093 | 107.75M | 0.136 | 1.41× | 1.47× |
| 100,000 | 1.108 | 90.23M | 1.039 | 96.21M | 1.126 | 1.02× | 1.08× |
| 1,000,000 | 16.638 | 60.10M | 10.326 | 96.85M | 10.047 | 0.60× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.134 | 1.47× |
| 1 | 5 | 0.324 | 0.511 | 1.57× |
| 1 | 10 | 0.456 | 1.037 | 2.28× |
| 10 | 1 | 0.062 | 0.116 | 1.86× |
| 10 | 5 | 0.282 | 0.533 | 1.89× |
| 10 | 10 | 0.528 | 0.983 | 1.86× |
| 100 | 1 | 0.049 | 0.096 | 1.98× |
| 100 | 5 | 0.283 | 0.514 | 1.82× |
| 100 | 10 | 0.529 | 1.028 | 1.94× |
| 1,000 | 1 | 0.061 | 0.114 | 1.88× |
| 1,000 | 5 | 0.256 | 0.537 | 2.09× |
| 1,000 | 10 | 0.631 | 1.109 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
