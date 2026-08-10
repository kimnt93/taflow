# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.54M | 0.024 | 41.57M | 0.055 | 2.22× | 2.27× |
| 10,000 | 0.210 | 47.53M | 0.207 | 48.39M | 0.247 | 1.17× | 1.19× |
| 100,000 | 2.010 | 49.75M | 2.008 | 49.79M | 2.164 | 1.08× | 1.08× |
| 1,000,000 | 21.689 | 46.11M | 20.781 | 48.12M | 22.338 | 1.03× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.101 | 1.24× |
| 1 | 5 | 0.414 | 0.515 | 1.24× |
| 1 | 10 | 0.562 | 1.008 | 1.79× |
| 10 | 1 | 0.055 | 0.089 | 1.63× |
| 10 | 5 | 0.257 | 0.460 | 1.79× |
| 10 | 10 | 0.543 | 0.992 | 1.83× |
| 100 | 1 | 0.052 | 0.108 | 2.08× |
| 100 | 5 | 0.271 | 0.511 | 1.88× |
| 100 | 10 | 0.623 | 1.027 | 1.65× |
| 1,000 | 1 | 0.075 | 0.111 | 1.49× |
| 1,000 | 5 | 0.300 | 0.643 | 2.14× |
| 1,000 | 10 | 0.588 | 1.295 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
