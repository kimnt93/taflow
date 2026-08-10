# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.31M | 0.013 | 75.79M | 0.205 | 14.20× | 15.52× |
| 10,000 | 0.102 | 97.83M | 0.100 | 100.46M | 0.825 | 8.07× | 8.29× |
| 100,000 | 0.977 | 102.33M | 0.959 | 104.26M | 7.268 | 7.44× | 7.58× |
| 1,000,000 | 10.263 | 97.44M | 9.997 | 100.03M | 76.012 | 7.41× | 7.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.282 | 2.88× |
| 1 | 5 | 0.300 | 1.228 | 4.09× |
| 1 | 10 | 0.487 | 2.293 | 4.70× |
| 10 | 1 | 0.057 | 0.210 | 3.70× |
| 10 | 5 | 0.238 | 1.238 | 5.20× |
| 10 | 10 | 0.503 | 2.359 | 4.69× |
| 100 | 1 | 0.055 | 0.232 | 4.23× |
| 100 | 5 | 0.262 | 1.273 | 4.87× |
| 100 | 10 | 0.501 | 2.423 | 4.84× |
| 1,000 | 1 | 0.062 | 0.287 | 4.60× |
| 1,000 | 5 | 0.272 | 1.620 | 5.95× |
| 1,000 | 10 | 0.583 | 3.172 | 5.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
