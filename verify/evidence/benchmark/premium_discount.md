# PremiumDiscount benchmark (`rolling premium-discount zone` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.30M | 0.021 | 47.71M | 3.259 | 147.62× | 155.48× |
| 10,000 | 0.266 | 37.59M | 0.254 | 39.34M | 32.315 | 121.48× | 127.13× |
| 100,000 | 2.506 | 39.90M | 2.776 | 36.02M | 330.985 | 132.07× | 119.23× |
| 1,000,000 | 27.523 | 36.33M | 25.771 | 38.80M | 3297.731 | 119.82× | 127.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.141 | 0.80× |
| 1 | 5 | 0.309 | 0.487 | 1.58× |
| 1 | 10 | 0.477 | 0.985 | 2.06× |
| 10 | 1 | 0.052 | 0.142 | 2.71× |
| 10 | 5 | 0.231 | 0.619 | 2.67× |
| 10 | 10 | 0.488 | 1.258 | 2.58× |
| 100 | 1 | 0.051 | 0.420 | 8.20× |
| 100 | 5 | 0.229 | 2.052 | 8.94× |
| 100 | 10 | 0.492 | 4.138 | 8.41× |
| 1,000 | 1 | 0.084 | 3.371 | 40.28× |
| 1,000 | 5 | 0.250 | 16.760 | 67.00× |
| 1,000 | 10 | 0.561 | 36.378 | 64.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
