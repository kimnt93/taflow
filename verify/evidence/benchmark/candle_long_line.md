# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.090 | 11.10M | 0.081 | 12.29M | 0.034 | 0.38× | 0.42× |
| 10,000 | 0.700 | 14.28M | 0.699 | 14.31M | 0.167 | 0.24× | 0.24× |
| 100,000 | 7.206 | 13.88M | 7.000 | 14.29M | 1.547 | 0.21× | 0.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.111 | 1.01× |
| 1 | 5 | 0.446 | 0.465 | 1.04× |
| 1 | 10 | 0.634 | 0.929 | 1.47× |
| 10 | 1 | 0.069 | 0.087 | 1.27× |
| 10 | 5 | 0.309 | 0.431 | 1.40× |
| 10 | 10 | 0.630 | 0.905 | 1.44× |
| 100 | 1 | 0.077 | 0.092 | 1.19× |
| 100 | 5 | 0.303 | 0.445 | 1.47× |
| 100 | 10 | 0.674 | 0.924 | 1.37× |
| 1,000 | 1 | 0.144 | 0.108 | 0.75× |
| 1,000 | 5 | 0.361 | 0.504 | 1.39× |
| 1,000 | 10 | 0.728 | 1.057 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
