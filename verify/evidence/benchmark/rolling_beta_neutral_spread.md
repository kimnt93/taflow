# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.93M | 0.047 | 21.37M | 0.208 | 4.15× | 4.45× |
| 10,000 | 0.481 | 20.81M | 0.475 | 21.07M | 0.938 | 1.95× | 1.98× |
| 100,000 | 4.830 | 20.70M | 4.537 | 22.04M | 8.133 | 1.68× | 1.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.247 | 3.29× |
| 1 | 5 | 0.283 | 1.047 | 3.69× |
| 1 | 10 | 0.433 | 2.237 | 5.16× |
| 10 | 1 | 0.048 | 0.208 | 4.30× |
| 10 | 5 | 0.188 | 1.260 | 6.72× |
| 10 | 10 | 0.412 | 2.250 | 5.46× |
| 100 | 1 | 0.049 | 0.222 | 4.51× |
| 100 | 5 | 0.212 | 1.306 | 6.18× |
| 100 | 10 | 0.443 | 2.322 | 5.24× |
| 1,000 | 1 | 0.094 | 0.310 | 3.29× |
| 1,000 | 5 | 0.233 | 1.682 | 7.21× |
| 1,000 | 10 | 0.447 | 3.239 | 7.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
