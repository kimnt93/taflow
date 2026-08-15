# CypherPattern benchmark (`Cypher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.43M | 0.007 | 144.99M | 0.230 | 24.75× | 33.40× |
| 10,000 | 0.088 | 113.69M | 0.085 | 117.04M | 1.368 | 15.55× | 16.01× |
| 100,000 | 0.853 | 117.23M | 0.820 | 121.92M | 13.062 | 15.31× | 15.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.209 | 2.84× |
| 1 | 5 | 0.327 | 0.815 | 2.49× |
| 1 | 10 | 0.442 | 1.659 | 3.75× |
| 10 | 1 | 0.044 | 0.156 | 3.58× |
| 10 | 5 | 0.200 | 1.139 | 5.71× |
| 10 | 10 | 0.435 | 1.672 | 3.84× |
| 100 | 1 | 0.046 | 0.176 | 3.86× |
| 100 | 5 | 0.194 | 1.139 | 5.86× |
| 100 | 10 | 0.472 | 1.804 | 3.83× |
| 1,000 | 1 | 0.053 | 0.298 | 5.64× |
| 1,000 | 5 | 0.209 | 1.843 | 8.80× |
| 1,000 | 10 | 0.421 | 2.913 | 6.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
