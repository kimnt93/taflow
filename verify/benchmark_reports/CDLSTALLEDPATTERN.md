# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.01M | 0.009 | 117.33M | 0.040 | 3.96× | 4.74× |
| 10,000 | 0.072 | 139.64M | 0.069 | 144.67M | 0.158 | 2.21× | 2.29× |
| 100,000 | 0.748 | 133.67M | 0.721 | 138.60M | 1.345 | 1.80× | 1.86× |
| 1,000,000 | 7.924 | 126.20M | 7.959 | 125.64M | 13.810 | 1.74× | 1.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.127 | 1.10× |
| 1 | 5 | 0.298 | 0.534 | 1.79× |
| 1 | 10 | 0.561 | 0.964 | 1.72× |
| 10 | 1 | 0.057 | 0.094 | 1.66× |
| 10 | 5 | 0.265 | 0.498 | 1.88× |
| 10 | 10 | 0.576 | 1.019 | 1.77× |
| 100 | 1 | 0.058 | 0.094 | 1.61× |
| 100 | 5 | 0.293 | 0.523 | 1.79× |
| 100 | 10 | 0.574 | 1.023 | 1.78× |
| 1,000 | 1 | 0.070 | 0.114 | 1.63× |
| 1,000 | 5 | 0.281 | 0.554 | 1.97× |
| 1,000 | 10 | 0.595 | 1.165 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
