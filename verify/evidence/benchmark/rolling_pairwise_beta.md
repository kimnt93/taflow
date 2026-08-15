# RollingPairwiseBeta benchmark (`PairwiseBeta` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.74M | 0.033 | 29.98M | 0.220 | 6.31× | 6.58× |
| 10,000 | 0.330 | 30.35M | 0.330 | 30.33M | 0.977 | 2.96× | 2.96× |
| 100,000 | 3.214 | 31.12M | 3.250 | 30.77M | 9.310 | 2.90× | 2.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.234 | 2.90× |
| 1 | 5 | 0.307 | 1.114 | 3.62× |
| 1 | 10 | 0.402 | 2.211 | 5.50× |
| 10 | 1 | 0.047 | 0.210 | 4.50× |
| 10 | 5 | 0.233 | 1.304 | 5.61× |
| 10 | 10 | 0.405 | 2.282 | 5.63× |
| 100 | 1 | 0.055 | 0.225 | 4.12× |
| 100 | 5 | 0.221 | 1.257 | 5.69× |
| 100 | 10 | 0.429 | 2.293 | 5.35× |
| 1,000 | 1 | 0.126 | 0.295 | 2.35× |
| 1,000 | 5 | 0.203 | 1.642 | 8.08× |
| 1,000 | 10 | 0.454 | 3.157 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
