# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.75M | 0.015 | 68.27M | 0.038 | 2.06× | 2.56× |
| 10,000 | 0.117 | 85.38M | 0.115 | 87.06M | 0.153 | 1.31× | 1.33× |
| 100,000 | 1.160 | 86.24M | 1.128 | 88.65M | 0.839 | 0.72× | 0.74× |
| 1,000,000 | 11.766 | 84.99M | 11.475 | 87.15M | 8.425 | 0.72× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.137 | 1.59× |
| 1 | 5 | 0.339 | 0.442 | 1.30× |
| 1 | 10 | 0.532 | 1.003 | 1.88× |
| 10 | 1 | 0.063 | 0.112 | 1.78× |
| 10 | 5 | 0.251 | 0.449 | 1.79× |
| 10 | 10 | 0.529 | 0.944 | 1.78× |
| 100 | 1 | 0.057 | 0.099 | 1.74× |
| 100 | 5 | 0.295 | 0.525 | 1.78× |
| 100 | 10 | 0.532 | 0.903 | 1.70× |
| 1,000 | 1 | 0.063 | 0.095 | 1.50× |
| 1,000 | 5 | 0.278 | 0.509 | 1.83× |
| 1,000 | 10 | 0.621 | 1.089 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
