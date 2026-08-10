# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.13M | 0.017 | 59.75M | 0.048 | 2.38× | 2.89× |
| 10,000 | 0.182 | 55.03M | 0.166 | 60.18M | 0.194 | 1.07× | 1.17× |
| 100,000 | 1.880 | 53.18M | 1.822 | 54.87M | 1.597 | 0.85× | 0.88× |
| 1,000,000 | 18.021 | 55.49M | 19.875 | 50.32M | 16.525 | 0.92× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.174 | 2.10× |
| 1 | 5 | 0.415 | 0.758 | 1.83× |
| 1 | 10 | 0.712 | 1.226 | 1.72× |
| 10 | 1 | 0.075 | 0.112 | 1.49× |
| 10 | 5 | 0.354 | 0.536 | 1.51× |
| 10 | 10 | 0.664 | 1.293 | 1.95× |
| 100 | 1 | 0.074 | 0.138 | 1.85× |
| 100 | 5 | 0.365 | 0.558 | 1.53× |
| 100 | 10 | 0.708 | 1.239 | 1.75× |
| 1,000 | 1 | 0.095 | 0.141 | 1.48× |
| 1,000 | 5 | 0.339 | 0.676 | 2.00× |
| 1,000 | 10 | 0.715 | 1.465 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
