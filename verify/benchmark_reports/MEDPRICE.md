# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 201.36M | 0.004 | 275.08M | 0.031 | 6.26× | 8.55× |
| 10,000 | 0.010 | 955.67M | 0.009 | 1.18G | 0.035 | 3.39× | 4.17× |
| 100,000 | 0.068 | 1.48G | 0.044 | 2.26G | 0.069 | 1.03× | 1.57× |
| 1,000,000 | 1.122 | 891.58M | 1.072 | 933.07M | 0.915 | 0.82× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.123 | 1.19× |
| 1 | 5 | 0.299 | 0.457 | 1.53× |
| 1 | 10 | 0.476 | 0.920 | 1.93× |
| 10 | 1 | 0.052 | 0.095 | 1.84× |
| 10 | 5 | 0.251 | 0.458 | 1.83× |
| 10 | 10 | 0.499 | 0.921 | 1.85× |
| 100 | 1 | 0.048 | 0.089 | 1.85× |
| 100 | 5 | 0.213 | 0.437 | 2.05× |
| 100 | 10 | 0.511 | 0.960 | 1.88× |
| 1,000 | 1 | 0.052 | 0.089 | 1.72× |
| 1,000 | 5 | 0.256 | 0.486 | 1.90× |
| 1,000 | 10 | 0.594 | 1.129 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
