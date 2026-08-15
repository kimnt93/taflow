# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.10M | 0.005 | 188.56M | 0.039 | 5.69× | 7.29× |
| 10,000 | 0.046 | 217.63M | 0.043 | 234.26M | 0.104 | 2.27× | 2.44× |
| 100,000 | 0.475 | 210.64M | 0.428 | 233.67M | 0.725 | 1.53× | 1.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.146 | 0.167 | 1.14× |
| 1 | 5 | 0.315 | 0.469 | 1.49× |
| 1 | 10 | 0.390 | 0.981 | 2.51× |
| 10 | 1 | 0.053 | 0.097 | 1.84× |
| 10 | 5 | 0.194 | 0.438 | 2.26× |
| 10 | 10 | 0.398 | 0.956 | 2.40× |
| 100 | 1 | 0.046 | 0.089 | 1.93× |
| 100 | 5 | 0.213 | 0.505 | 2.37× |
| 100 | 10 | 0.412 | 0.922 | 2.24× |
| 1,000 | 1 | 0.047 | 0.096 | 2.06× |
| 1,000 | 5 | 0.199 | 0.474 | 2.38× |
| 1,000 | 10 | 0.487 | 1.058 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
