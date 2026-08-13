# PremiumDiscount benchmark (`rolling premium-discount zone` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.51M | 0.095 | 10.56M | 3.095 | 29.44× | 32.70× |
| 10,000 | 1.000 | 10.00M | 0.906 | 11.03M | 31.280 | 31.28× | 34.51× |
| 100,000 | 9.128 | 10.96M | 9.057 | 11.04M | 314.757 | 34.48× | 34.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.137 | 1.37× |
| 1 | 5 | 0.408 | 0.474 | 1.16× |
| 1 | 10 | 0.622 | 0.968 | 1.56× |
| 10 | 1 | 0.064 | 0.125 | 1.96× |
| 10 | 5 | 0.297 | 0.614 | 2.06× |
| 10 | 10 | 0.582 | 1.246 | 2.14× |
| 100 | 1 | 0.083 | 0.408 | 4.89× |
| 100 | 5 | 0.302 | 2.058 | 6.82× |
| 100 | 10 | 0.624 | 4.116 | 6.60× |
| 1,000 | 1 | 0.177 | 3.280 | 18.54× |
| 1,000 | 5 | 0.375 | 16.700 | 44.50× |
| 1,000 | 10 | 0.826 | 35.601 | 43.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
