# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.32M | 0.004 | 245.25M | 0.028 | 5.22× | 6.79× |
| 10,000 | 0.025 | 402.39M | 0.021 | 470.80M | 0.040 | 1.60× | 1.87× |
| 100,000 | 0.216 | 463.48M | 0.207 | 482.75M | 0.163 | 0.75× | 0.78× |
| 1,000,000 | 3.055 | 327.37M | 2.491 | 401.51M | 1.516 | 0.50× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.118 | 0.86× |
| 1 | 5 | 0.309 | 0.445 | 1.44× |
| 1 | 10 | 0.491 | 0.920 | 1.88× |
| 10 | 1 | 0.054 | 0.101 | 1.86× |
| 10 | 5 | 0.264 | 0.482 | 1.83× |
| 10 | 10 | 0.481 | 0.894 | 1.86× |
| 100 | 1 | 0.049 | 0.085 | 1.74× |
| 100 | 5 | 0.242 | 0.422 | 1.75× |
| 100 | 10 | 0.481 | 0.924 | 1.92× |
| 1,000 | 1 | 0.053 | 0.095 | 1.78× |
| 1,000 | 5 | 0.250 | 0.428 | 1.71× |
| 1,000 | 10 | 0.480 | 0.918 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
