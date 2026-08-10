# RollingInterquartileRange benchmark (`RollingIqr` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.81M | 0.073 | 13.75M | 0.449 | 6.19× | 6.17× |
| 10,000 | 0.690 | 14.48M | 0.662 | 15.10M | 1.722 | 2.49× | 2.60× |
| 100,000 | 6.663 | 15.01M | 7.110 | 14.06M | 21.085 | 3.16× | 2.97× |
| 1,000,000 | 68.628 | 14.57M | 66.179 | 15.11M | 153.821 | 2.24× | 2.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.590 | 5.15× |
| 1 | 5 | 0.321 | 1.100 | 3.42× |
| 1 | 10 | 0.498 | 2.650 | 5.32× |
| 10 | 1 | 0.048 | 0.204 | 4.28× |
| 10 | 5 | 0.243 | 1.037 | 4.28× |
| 10 | 10 | 0.468 | 2.301 | 4.92× |
| 100 | 1 | 0.057 | 0.227 | 4.03× |
| 100 | 5 | 0.237 | 1.420 | 5.99× |
| 100 | 10 | 0.482 | 2.486 | 5.16× |
| 1,000 | 1 | 0.126 | 0.386 | 3.07× |
| 1,000 | 5 | 0.277 | 2.197 | 7.94× |
| 1,000 | 10 | 0.517 | 4.021 | 7.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
