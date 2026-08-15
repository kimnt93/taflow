# PremiumDiscount benchmark (`rolling premium-discount zone` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.60M | 0.019 | 53.95M | 3.307 | 167.36× | 178.45× |
| 10,000 | 0.261 | 38.32M | 0.257 | 38.98M | 32.781 | 125.60× | 127.80× |
| 100,000 | 2.829 | 35.35M | 2.528 | 39.56M | 333.719 | 117.97× | 132.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.145 | 1.18× |
| 1 | 5 | 0.323 | 0.591 | 1.83× |
| 1 | 10 | 0.388 | 0.975 | 2.52× |
| 10 | 1 | 0.046 | 0.127 | 2.78× |
| 10 | 5 | 0.196 | 0.619 | 3.15× |
| 10 | 10 | 0.408 | 1.301 | 3.19× |
| 100 | 1 | 0.049 | 0.442 | 9.07× |
| 100 | 5 | 0.206 | 2.123 | 10.32× |
| 100 | 10 | 0.441 | 4.256 | 9.65× |
| 1,000 | 1 | 0.075 | 3.506 | 46.45× |
| 1,000 | 5 | 0.219 | 18.750 | 85.71× |
| 1,000 | 10 | 0.574 | 55.906 | 97.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
