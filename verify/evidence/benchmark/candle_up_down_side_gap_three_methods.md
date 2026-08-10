# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.08M | 0.013 | 74.91M | 0.031 | 2.03× | 2.30× |
| 10,000 | 0.109 | 91.79M | 0.105 | 95.06M | 0.086 | 0.79× | 0.82× |
| 100,000 | 1.060 | 94.35M | 1.023 | 97.77M | 0.586 | 0.55× | 0.57× |
| 1,000,000 | 10.538 | 94.89M | 10.197 | 98.07M | 5.640 | 0.54× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.109 | 1.05× |
| 1 | 5 | 0.323 | 0.443 | 1.37× |
| 1 | 10 | 0.479 | 0.906 | 1.89× |
| 10 | 1 | 0.061 | 0.093 | 1.52× |
| 10 | 5 | 0.233 | 0.427 | 1.83× |
| 10 | 10 | 0.503 | 0.929 | 1.85× |
| 100 | 1 | 0.056 | 0.099 | 1.76× |
| 100 | 5 | 0.238 | 0.435 | 1.83× |
| 100 | 10 | 0.507 | 0.905 | 1.78× |
| 1,000 | 1 | 0.065 | 0.097 | 1.48× |
| 1,000 | 5 | 0.244 | 0.460 | 1.89× |
| 1,000 | 10 | 0.526 | 0.977 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
