# BullishPercentIndex benchmark (`BullishPercentIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 207.56M | 0.004 | 276.69M | 12.057 | 2502.69× | 3336.21× |
| 10,000 | 0.030 | 332.95M | 0.025 | 399.10M | 116.693 | 3885.31× | 4657.24× |
| 100,000 | 0.270 | 371.00M | 0.233 | 429.34M | 1145.201 | 4248.75× | 4916.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.295 | 2.29× |
| 1 | 5 | 0.283 | 1.058 | 3.75× |
| 1 | 10 | 0.404 | 2.108 | 5.22× |
| 10 | 1 | 0.041 | 0.311 | 7.61× |
| 10 | 5 | 0.189 | 1.929 | 10.23× |
| 10 | 10 | 0.385 | 3.214 | 8.35× |
| 100 | 1 | 0.049 | 1.361 | 27.77× |
| 100 | 5 | 0.186 | 7.217 | 38.72× |
| 100 | 10 | 0.457 | 14.023 | 30.68× |
| 1,000 | 1 | 0.052 | 11.885 | 227.98× |
| 1,000 | 5 | 0.444 | 60.208 | 135.70× |
| 1,000 | 10 | 0.524 | 118.767 | 226.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
