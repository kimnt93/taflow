# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.77M | 0.008 | 124.90M | 0.153 | 17.81× | 19.05× |
| 10,000 | 0.058 | 173.86M | 0.056 | 178.05M | 0.470 | 8.17× | 8.36× |
| 100,000 | 0.559 | 179.00M | 0.538 | 186.04M | 3.627 | 6.49× | 6.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.248 | 1.86× |
| 1 | 5 | 0.400 | 0.974 | 2.44× |
| 1 | 10 | 0.547 | 2.357 | 4.31× |
| 10 | 1 | 0.052 | 0.198 | 3.81× |
| 10 | 5 | 0.247 | 1.034 | 4.18× |
| 10 | 10 | 0.523 | 2.121 | 4.05× |
| 100 | 1 | 0.051 | 0.192 | 3.77× |
| 100 | 5 | 0.259 | 1.011 | 3.90× |
| 100 | 10 | 0.497 | 2.213 | 4.45× |
| 1,000 | 1 | 0.068 | 0.247 | 3.65× |
| 1,000 | 5 | 0.235 | 1.142 | 4.85× |
| 1,000 | 10 | 0.521 | 2.559 | 4.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
