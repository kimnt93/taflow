# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.127 | 7.89M | 0.120 | 8.35M | 0.120 | 0.95× | 1.00× |
| 10,000 | 1.218 | 8.21M | 1.145 | 8.73M | 0.781 | 0.64× | 0.68× |
| 100,000 | 12.246 | 8.17M | 11.794 | 8.48M | 7.532 | 0.62× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.170 | 1.31× |
| 1 | 5 | 0.345 | 0.540 | 1.57× |
| 1 | 10 | 0.522 | 1.064 | 2.04× |
| 10 | 1 | 0.055 | 0.114 | 2.09× |
| 10 | 5 | 0.285 | 0.515 | 1.81× |
| 10 | 10 | 0.544 | 1.001 | 1.84× |
| 100 | 1 | 0.062 | 0.106 | 1.69× |
| 100 | 5 | 0.252 | 0.516 | 2.05× |
| 100 | 10 | 0.556 | 1.093 | 1.97× |
| 1,000 | 1 | 0.169 | 0.182 | 1.07× |
| 1,000 | 5 | 0.402 | 0.907 | 2.26× |
| 1,000 | 10 | 0.725 | 1.942 | 2.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
