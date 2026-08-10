# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.29M | 0.006 | 163.44M | 0.038 | 5.29× | 6.25× |
| 10,000 | 0.053 | 189.06M | 0.049 | 202.18M | 0.092 | 1.74× | 1.86× |
| 100,000 | 0.539 | 185.50M | 0.564 | 177.36M | 0.715 | 1.33× | 1.27× |
| 1,000,000 | 5.350 | 186.91M | 4.975 | 200.99M | 6.481 | 1.21× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.105 | 0.90× |
| 1 | 5 | 0.347 | 0.523 | 1.51× |
| 1 | 10 | 0.479 | 0.923 | 1.93× |
| 10 | 1 | 0.052 | 0.090 | 1.75× |
| 10 | 5 | 0.225 | 0.450 | 2.00× |
| 10 | 10 | 0.490 | 0.965 | 1.97× |
| 100 | 1 | 0.052 | 0.099 | 1.90× |
| 100 | 5 | 0.235 | 0.449 | 1.91× |
| 100 | 10 | 0.483 | 0.912 | 1.89× |
| 1,000 | 1 | 0.052 | 0.095 | 1.81× |
| 1,000 | 5 | 0.224 | 0.464 | 2.07× |
| 1,000 | 10 | 0.468 | 1.036 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
