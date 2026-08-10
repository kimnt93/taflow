# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.08M | 0.024 | 42.27M | 0.283 | 10.49× | 11.96× |
| 10,000 | 0.186 | 53.72M | 0.182 | 54.83M | 1.573 | 8.45× | 8.62× |
| 100,000 | 1.809 | 55.27M | 1.746 | 57.26M | 14.550 | 8.04× | 8.33× |
| 1,000,000 | 18.777 | 53.26M | 18.220 | 54.88M | 141.920 | 7.56× | 7.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.547 | 5.87× |
| 1 | 5 | 0.323 | 1.210 | 3.74× |
| 1 | 10 | 0.542 | 2.486 | 4.58× |
| 10 | 1 | 0.060 | 0.219 | 3.64× |
| 10 | 5 | 0.307 | 1.371 | 4.47× |
| 10 | 10 | 0.565 | 2.719 | 4.81× |
| 100 | 1 | 0.072 | 0.237 | 3.29× |
| 100 | 5 | 0.297 | 1.443 | 4.86× |
| 100 | 10 | 0.561 | 2.783 | 4.96× |
| 1,000 | 1 | 0.096 | 0.375 | 3.89× |
| 1,000 | 5 | 0.304 | 2.139 | 7.04× |
| 1,000 | 10 | 0.631 | 4.067 | 6.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
