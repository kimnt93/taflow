# ExponentiallyWeightedCovariance benchmark (`ewm covariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.28M | 0.009 | 117.19M | 1.271 | 126.14× | 148.89× |
| 10,000 | 0.053 | 187.28M | 0.050 | 199.01M | 12.537 | 234.79× | 249.50× |
| 100,000 | 0.497 | 201.28M | 0.478 | 209.14M | 128.369 | 258.38× | 268.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.147 | 1.29× |
| 1 | 5 | 0.410 | 0.433 | 1.06× |
| 1 | 10 | 0.519 | 0.951 | 1.83× |
| 10 | 1 | 0.052 | 0.098 | 1.88× |
| 10 | 5 | 0.248 | 0.476 | 1.91× |
| 10 | 10 | 0.499 | 1.093 | 2.19× |
| 100 | 1 | 0.076 | 0.233 | 3.06× |
| 100 | 5 | 0.248 | 1.071 | 4.32× |
| 100 | 10 | 0.518 | 2.282 | 4.40× |
| 1,000 | 1 | 0.059 | 1.388 | 23.44× |
| 1,000 | 5 | 0.242 | 6.906 | 28.59× |
| 1,000 | 10 | 0.612 | 14.794 | 24.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
