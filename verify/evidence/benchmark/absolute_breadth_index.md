# AbsoluteBreadthIndex benchmark (`AbsoluteBreadthIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.30M | 0.005 | 193.35M | 8.319 | 1300.29× | 1608.52× |
| 10,000 | 0.029 | 345.35M | 0.026 | 387.59M | 78.751 | 2719.67× | 3052.32× |
| 100,000 | 0.229 | 436.20M | 0.223 | 448.60M | 802.122 | 3498.87× | 3598.28× |
| 1,000,000 | 3.293 | 303.67M | 2.565 | 389.86M | 8021.643 | 2435.96× | 3127.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.256 | 1.97× |
| 1 | 5 | 0.260 | 1.099 | 4.22× |
| 1 | 10 | 0.502 | 2.182 | 4.35× |
| 10 | 1 | 0.050 | 0.289 | 5.77× |
| 10 | 5 | 0.232 | 1.677 | 7.24× |
| 10 | 10 | 0.490 | 2.919 | 5.96× |
| 100 | 1 | 0.051 | 1.057 | 20.80× |
| 100 | 5 | 0.238 | 5.815 | 24.38× |
| 100 | 10 | 0.491 | 10.911 | 22.22× |
| 1,000 | 1 | 0.059 | 8.744 | 149.03× |
| 1,000 | 5 | 0.435 | 44.036 | 101.13× |
| 1,000 | 10 | 0.589 | 95.023 | 161.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
