# NewHighsNewLows benchmark (`NewHighsNewLows` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.81M | 0.003 | 306.75M | 8.304 | 1775.49× | 2547.27× |
| 10,000 | 0.027 | 372.34M | 0.024 | 424.36M | 83.220 | 3098.58× | 3531.47× |
| 100,000 | 0.245 | 407.70M | 0.222 | 450.07M | 826.114 | 3368.05× | 3718.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.276 | 2.11× |
| 1 | 5 | 0.238 | 1.452 | 6.11× |
| 1 | 10 | 0.400 | 2.036 | 5.09× |
| 10 | 1 | 0.044 | 0.281 | 6.45× |
| 10 | 5 | 0.195 | 1.742 | 8.95× |
| 10 | 10 | 0.381 | 2.888 | 7.59× |
| 100 | 1 | 0.046 | 1.038 | 22.48× |
| 100 | 5 | 0.206 | 5.736 | 27.91× |
| 100 | 10 | 0.372 | 10.671 | 28.68× |
| 1,000 | 1 | 0.055 | 8.583 | 156.93× |
| 1,000 | 5 | 0.314 | 48.053 | 153.20× |
| 1,000 | 10 | 0.444 | 89.320 | 201.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
