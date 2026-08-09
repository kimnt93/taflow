# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.56M | 0.008 | 129.31M | 0.043 | 4.52× | 5.54× |
| 10,000 | 0.103 | 96.86M | 0.100 | 99.69M | 0.160 | 1.55× | 1.59× |
| 100,000 | 1.133 | 88.29M | 1.119 | 89.33M | 1.384 | 1.22× | 1.24× |
| 1,000,000 | 11.995 | 83.37M | 11.728 | 85.27M | 13.546 | 1.13× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.108 | 1.08× |
| 1 | 5 | 0.339 | 0.551 | 1.62× |
| 1 | 10 | 0.534 | 0.945 | 1.77× |
| 10 | 1 | 0.052 | 0.092 | 1.77× |
| 10 | 5 | 0.245 | 0.451 | 1.84× |
| 10 | 10 | 0.510 | 0.927 | 1.82× |
| 100 | 1 | 0.057 | 0.092 | 1.61× |
| 100 | 5 | 0.246 | 0.449 | 1.83× |
| 100 | 10 | 0.535 | 0.930 | 1.74× |
| 1,000 | 1 | 0.068 | 0.108 | 1.59× |
| 1,000 | 5 | 0.259 | 0.522 | 2.02× |
| 1,000 | 10 | 0.550 | 1.092 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
