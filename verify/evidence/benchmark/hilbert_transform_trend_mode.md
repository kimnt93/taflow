# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.169 | 5.91M | 0.168 | 5.94M | 0.497 | 2.94× | 2.95× |
| 10,000 | 1.900 | 5.26M | 1.801 | 5.55M | 5.014 | 2.64× | 2.78× |
| 100,000 | 17.417 | 5.74M | 16.946 | 5.90M | 47.541 | 2.73× | 2.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.246 | 0.145 | 0.59× |
| 1 | 5 | 0.203 | 0.454 | 2.24× |
| 1 | 10 | 0.381 | 0.930 | 2.44× |
| 10 | 1 | 0.050 | 0.100 | 2.01× |
| 10 | 5 | 0.197 | 0.431 | 2.18× |
| 10 | 10 | 0.406 | 0.896 | 2.20× |
| 100 | 1 | 0.058 | 0.117 | 2.01× |
| 100 | 5 | 0.196 | 0.586 | 2.99× |
| 100 | 10 | 0.439 | 1.193 | 2.71× |
| 1,000 | 1 | 0.235 | 0.567 | 2.41× |
| 1,000 | 5 | 0.357 | 2.893 | 8.11× |
| 1,000 | 10 | 0.586 | 5.760 | 9.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
