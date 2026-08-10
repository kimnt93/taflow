# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.82M | 0.007 | 134.83M | 0.037 | 3.69× | 5.03× |
| 10,000 | 0.077 | 129.62M | 0.080 | 124.88M | 0.099 | 1.29× | 1.24× |
| 100,000 | 0.753 | 132.82M | 0.725 | 137.86M | 0.682 | 0.91× | 0.94× |
| 1,000,000 | 8.683 | 115.17M | 8.112 | 123.28M | 6.861 | 0.79× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.115 | 1.04× |
| 1 | 5 | 0.283 | 0.455 | 1.61× |
| 1 | 10 | 0.480 | 1.009 | 2.10× |
| 10 | 1 | 0.053 | 0.094 | 1.76× |
| 10 | 5 | 0.214 | 0.428 | 2.00× |
| 10 | 10 | 0.479 | 0.924 | 1.93× |
| 100 | 1 | 0.048 | 0.098 | 2.05× |
| 100 | 5 | 0.220 | 0.435 | 1.98× |
| 100 | 10 | 0.476 | 0.902 | 1.89× |
| 1,000 | 1 | 0.059 | 0.100 | 1.70× |
| 1,000 | 5 | 0.238 | 0.475 | 2.00× |
| 1,000 | 10 | 0.476 | 1.015 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
