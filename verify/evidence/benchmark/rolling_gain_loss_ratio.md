# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.08M | 0.035 | 28.52M | 0.168 | 5.57× | 4.81× |
| 10,000 | 0.268 | 37.29M | 0.264 | 37.86M | 0.775 | 2.89× | 2.93× |
| 100,000 | 2.810 | 35.59M | 2.792 | 35.81M | 5.542 | 1.97× | 1.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.282 | 3.78× |
| 1 | 5 | 0.269 | 1.113 | 4.14× |
| 1 | 10 | 0.560 | 2.405 | 4.30× |
| 10 | 1 | 0.051 | 0.197 | 3.84× |
| 10 | 5 | 0.290 | 1.052 | 3.63× |
| 10 | 10 | 0.484 | 2.183 | 4.51× |
| 100 | 1 | 0.058 | 0.221 | 3.83× |
| 100 | 5 | 0.256 | 0.985 | 3.84× |
| 100 | 10 | 0.475 | 2.403 | 5.06× |
| 1,000 | 1 | 0.078 | 0.250 | 3.18× |
| 1,000 | 5 | 0.266 | 1.245 | 4.69× |
| 1,000 | 10 | 0.537 | 2.757 | 5.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
