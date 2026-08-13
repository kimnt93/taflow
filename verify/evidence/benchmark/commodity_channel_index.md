# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.309 | 3.24M | 0.316 | 3.16M | 0.051 | 0.16× | 0.16× |
| 10,000 | 2.904 | 3.44M | 3.019 | 3.31M | 0.225 | 0.08× | 0.07× |
| 100,000 | 30.043 | 3.33M | 29.992 | 3.33M | 2.000 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.146 | 0.88× |
| 1 | 5 | 0.467 | 0.513 | 1.10× |
| 1 | 10 | 0.645 | 0.902 | 1.40× |
| 10 | 1 | 0.067 | 0.093 | 1.38× |
| 10 | 5 | 0.317 | 0.439 | 1.39× |
| 10 | 10 | 0.665 | 0.945 | 1.42× |
| 100 | 1 | 0.104 | 0.090 | 0.87× |
| 100 | 5 | 0.310 | 0.444 | 1.43× |
| 100 | 10 | 0.676 | 0.957 | 1.42× |
| 1,000 | 1 | 0.393 | 0.115 | 0.29× |
| 1,000 | 5 | 0.654 | 0.563 | 0.86× |
| 1,000 | 10 | 1.038 | 1.157 | 1.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
