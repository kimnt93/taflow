# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 9.95M | 0.108 | 9.29M | 0.426 | 4.24× | 3.95× |
| 10,000 | 0.976 | 10.25M | 0.991 | 10.09M | 4.280 | 4.39× | 4.32× |
| 100,000 | 9.912 | 10.09M | 9.754 | 10.25M | 41.299 | 4.17× | 4.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.108 | 1.59× |
| 1 | 5 | 0.316 | 0.438 | 1.39× |
| 1 | 10 | 0.457 | 0.928 | 2.03× |
| 10 | 1 | 0.049 | 0.086 | 1.75× |
| 10 | 5 | 0.228 | 0.423 | 1.86× |
| 10 | 10 | 0.473 | 0.879 | 1.86× |
| 100 | 1 | 0.058 | 0.115 | 1.96× |
| 100 | 5 | 0.237 | 0.561 | 2.37× |
| 100 | 10 | 0.501 | 1.143 | 2.28× |
| 1,000 | 1 | 0.150 | 0.522 | 3.47× |
| 1,000 | 5 | 0.313 | 2.589 | 8.27× |
| 1,000 | 10 | 0.548 | 5.231 | 9.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
