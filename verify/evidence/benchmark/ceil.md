# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 536.14M | 0.001 | 1.14G | 0.030 | 15.91× | 33.79× |
| 10,000 | 0.006 | 1.74G | 0.003 | 3.61G | 0.041 | 7.17× | 14.88× |
| 100,000 | 0.052 | 1.93G | 0.028 | 3.60G | 0.160 | 3.08× | 5.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.128 | 0.93× |
| 1 | 5 | 0.225 | 0.462 | 2.06× |
| 1 | 10 | 0.372 | 0.868 | 2.34× |
| 10 | 1 | 0.043 | 0.088 | 2.04× |
| 10 | 5 | 0.177 | 0.441 | 2.49× |
| 10 | 10 | 0.405 | 0.893 | 2.21× |
| 100 | 1 | 0.044 | 0.085 | 1.92× |
| 100 | 5 | 0.164 | 0.404 | 2.46× |
| 100 | 10 | 0.390 | 0.894 | 2.29× |
| 1,000 | 1 | 0.048 | 0.089 | 1.87× |
| 1,000 | 5 | 0.184 | 0.428 | 2.32× |
| 1,000 | 10 | 0.403 | 0.882 | 2.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
