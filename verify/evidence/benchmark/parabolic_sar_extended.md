# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.81M | 0.012 | 81.16M | 0.048 | 3.45× | 3.90× |
| 10,000 | 0.112 | 89.04M | 0.109 | 92.04M | 0.092 | 0.82× | 0.85× |
| 100,000 | 1.099 | 90.99M | 1.077 | 92.85M | 0.626 | 0.57× | 0.58× |
| 1,000,000 | 11.373 | 87.92M | 11.044 | 90.54M | 5.747 | 0.51× | 0.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.125 | 1.31× |
| 1 | 5 | 0.348 | 0.594 | 1.71× |
| 1 | 10 | 0.471 | 1.109 | 2.35× |
| 10 | 1 | 0.051 | 0.111 | 2.15× |
| 10 | 5 | 0.218 | 0.559 | 2.56× |
| 10 | 10 | 0.464 | 1.136 | 2.45× |
| 100 | 1 | 0.049 | 0.110 | 2.23× |
| 100 | 5 | 0.242 | 0.546 | 2.25× |
| 100 | 10 | 0.520 | 1.139 | 2.19× |
| 1,000 | 1 | 0.061 | 0.119 | 1.95× |
| 1,000 | 5 | 0.247 | 0.601 | 2.44× |
| 1,000 | 10 | 0.479 | 1.172 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
