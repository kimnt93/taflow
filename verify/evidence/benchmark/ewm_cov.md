# ExponentiallyWeightedCovariance benchmark (`ewm covariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.50M | 0.029 | 34.34M | 1.275 | 36.33× | 43.78× |
| 10,000 | 0.216 | 46.37M | 0.209 | 47.86M | 12.207 | 56.61× | 58.42× |
| 100,000 | 2.024 | 49.41M | 1.931 | 51.79M | 122.431 | 60.49× | 63.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.115 | 0.90× |
| 1 | 5 | 0.370 | 0.417 | 1.13× |
| 1 | 10 | 0.625 | 0.875 | 1.40× |
| 10 | 1 | 0.078 | 0.117 | 1.50× |
| 10 | 5 | 0.342 | 0.486 | 1.42× |
| 10 | 10 | 0.611 | 0.980 | 1.60× |
| 100 | 1 | 0.067 | 0.209 | 3.13× |
| 100 | 5 | 0.314 | 1.009 | 3.21× |
| 100 | 10 | 0.646 | 2.042 | 3.16× |
| 1,000 | 1 | 0.093 | 1.327 | 14.28× |
| 1,000 | 5 | 0.302 | 6.648 | 22.04× |
| 1,000 | 10 | 0.625 | 13.343 | 21.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
