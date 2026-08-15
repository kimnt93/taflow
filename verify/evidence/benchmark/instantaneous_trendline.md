# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.75M | 0.006 | 160.62M | 0.154 | 20.86× | 24.69× |
| 10,000 | 0.060 | 165.65M | 0.058 | 173.67M | 0.485 | 8.04× | 8.43× |
| 100,000 | 0.560 | 178.68M | 0.526 | 190.12M | 3.671 | 6.56× | 6.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.257 | 2.81× |
| 1 | 5 | 0.309 | 0.975 | 3.15× |
| 1 | 10 | 0.376 | 2.203 | 5.85× |
| 10 | 1 | 0.049 | 0.193 | 3.95× |
| 10 | 5 | 0.181 | 0.929 | 5.13× |
| 10 | 10 | 0.391 | 2.158 | 5.51× |
| 100 | 1 | 0.048 | 0.186 | 3.89× |
| 100 | 5 | 0.202 | 0.956 | 4.72× |
| 100 | 10 | 0.409 | 2.240 | 5.47× |
| 1,000 | 1 | 0.052 | 0.228 | 4.42× |
| 1,000 | 5 | 0.192 | 1.112 | 5.78× |
| 1,000 | 10 | 0.424 | 2.536 | 5.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
