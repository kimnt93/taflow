# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.42M | 0.012 | 81.57M | 0.041 | 3.12× | 3.33× |
| 10,000 | 0.104 | 95.93M | 0.101 | 98.78M | 0.121 | 1.16× | 1.20× |
| 100,000 | 1.021 | 97.94M | 1.089 | 91.83M | 0.930 | 0.91× | 0.85× |
| 1,000,000 | 10.450 | 95.70M | 10.124 | 98.78M | 9.923 | 0.95× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.182 | 1.56× |
| 1 | 5 | 0.335 | 0.475 | 1.41× |
| 1 | 10 | 0.482 | 0.948 | 1.97× |
| 10 | 1 | 0.046 | 0.088 | 1.92× |
| 10 | 5 | 0.233 | 0.537 | 2.31× |
| 10 | 10 | 0.585 | 0.983 | 1.68× |
| 100 | 1 | 0.052 | 0.096 | 1.86× |
| 100 | 5 | 0.230 | 0.458 | 1.99× |
| 100 | 10 | 0.490 | 0.949 | 1.94× |
| 1,000 | 1 | 0.057 | 0.103 | 1.81× |
| 1,000 | 5 | 0.249 | 0.480 | 1.93× |
| 1,000 | 10 | 0.469 | 1.081 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
