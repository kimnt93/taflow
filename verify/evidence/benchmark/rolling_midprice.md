# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 95.03M | 0.009 | 116.96M | 0.045 | 4.31× | 5.30× |
| 10,000 | 0.084 | 119.31M | 0.078 | 128.26M | 0.102 | 1.21× | 1.30× |
| 100,000 | 0.802 | 124.65M | 0.754 | 132.59M | 0.713 | 0.89× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.157 | 0.114 | 0.73× |
| 1 | 5 | 0.314 | 0.509 | 1.62× |
| 1 | 10 | 0.534 | 0.964 | 1.80× |
| 10 | 1 | 0.049 | 0.096 | 1.94× |
| 10 | 5 | 0.225 | 0.457 | 2.03× |
| 10 | 10 | 0.502 | 0.977 | 1.95× |
| 100 | 1 | 0.059 | 0.098 | 1.67× |
| 100 | 5 | 0.222 | 0.452 | 2.04× |
| 100 | 10 | 0.473 | 0.980 | 2.07× |
| 1,000 | 1 | 0.063 | 0.114 | 1.80× |
| 1,000 | 5 | 0.274 | 0.491 | 1.79× |
| 1,000 | 10 | 0.506 | 1.035 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
