# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.05M | 0.015 | 67.26M | 0.033 | 2.04× | 2.21× |
| 10,000 | 0.125 | 79.71M | 0.114 | 87.75M | 0.107 | 0.86× | 0.94× |
| 100,000 | 1.173 | 85.23M | 1.169 | 85.54M | 0.783 | 0.67× | 0.67× |
| 1,000,000 | 12.389 | 80.72M | 11.936 | 83.78M | 7.741 | 0.62× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.111 | 1.11× |
| 1 | 5 | 0.402 | 0.490 | 1.22× |
| 1 | 10 | 0.559 | 1.003 | 1.80× |
| 10 | 1 | 0.058 | 0.094 | 1.61× |
| 10 | 5 | 0.263 | 0.473 | 1.80× |
| 10 | 10 | 0.532 | 1.097 | 2.06× |
| 100 | 1 | 0.071 | 0.119 | 1.66× |
| 100 | 5 | 0.261 | 0.459 | 1.76× |
| 100 | 10 | 0.529 | 0.927 | 1.75× |
| 1,000 | 1 | 0.073 | 0.103 | 1.41× |
| 1,000 | 5 | 0.255 | 0.507 | 1.99× |
| 1,000 | 10 | 0.557 | 1.007 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
