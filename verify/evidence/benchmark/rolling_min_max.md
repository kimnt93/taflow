# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.01M | 0.008 | 125.73M | 0.044 | 4.82× | 5.56× |
| 10,000 | 0.073 | 137.86M | 0.070 | 142.68M | 0.117 | 1.61× | 1.67× |
| 100,000 | 0.739 | 135.34M | 0.693 | 144.22M | 0.857 | 1.16× | 1.24× |
| 1,000,000 | 9.069 | 110.26M | 8.390 | 119.19M | 8.510 | 0.94× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.152 | 2.31× |
| 1 | 5 | 0.310 | 0.509 | 1.64× |
| 1 | 10 | 0.481 | 1.045 | 2.17× |
| 10 | 1 | 0.062 | 0.107 | 1.71× |
| 10 | 5 | 0.223 | 0.461 | 2.07× |
| 10 | 10 | 0.465 | 0.975 | 2.10× |
| 100 | 1 | 0.050 | 0.094 | 1.86× |
| 100 | 5 | 0.233 | 0.484 | 2.07× |
| 100 | 10 | 0.537 | 0.983 | 1.83× |
| 1,000 | 1 | 0.058 | 0.108 | 1.87× |
| 1,000 | 5 | 0.249 | 0.541 | 2.17× |
| 1,000 | 10 | 0.511 | 1.121 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
