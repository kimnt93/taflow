# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.99M | 0.005 | 216.55M | 0.035 | 6.38× | 7.64× |
| 10,000 | 0.035 | 283.50M | 0.032 | 312.80M | 0.079 | 2.25× | 2.48× |
| 100,000 | 0.338 | 296.13M | 0.321 | 311.79M | 0.540 | 1.60× | 1.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.133 | 0.144 | 1.08× |
| 1 | 5 | 0.266 | 0.459 | 1.72× |
| 1 | 10 | 0.467 | 0.975 | 2.09× |
| 10 | 1 | 0.048 | 0.093 | 1.93× |
| 10 | 5 | 0.221 | 0.457 | 2.07× |
| 10 | 10 | 0.474 | 0.962 | 2.03× |
| 100 | 1 | 0.054 | 0.091 | 1.70× |
| 100 | 5 | 0.215 | 0.441 | 2.05× |
| 100 | 10 | 0.510 | 0.932 | 1.83× |
| 1,000 | 1 | 0.052 | 0.102 | 1.97× |
| 1,000 | 5 | 0.232 | 0.484 | 2.09× |
| 1,000 | 10 | 0.485 | 0.946 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
