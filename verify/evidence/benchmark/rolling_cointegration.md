# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.327 | 3.06M | 0.338 | 2.96M | 3.142 | 9.61× | 9.30× |
| 10,000 | 3.314 | 3.02M | 3.297 | 3.03M | 30.495 | 9.20× | 9.25× |
| 100,000 | 36.045 | 2.77M | 35.550 | 2.81M | 315.176 | 8.74× | 8.87× |
| 1,000,000 | 344.581 | 2.90M | 332.818 | 3.00M | 3138.590 | 9.11× | 9.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.404 | 5.21× |
| 1 | 5 | 0.443 | 1.244 | 2.81× |
| 1 | 10 | 0.491 | 2.602 | 5.30× |
| 10 | 1 | 0.059 | 0.250 | 4.24× |
| 10 | 5 | 0.227 | 1.304 | 5.76× |
| 10 | 10 | 0.540 | 2.668 | 4.94× |
| 100 | 1 | 0.077 | 0.497 | 6.49× |
| 100 | 5 | 0.246 | 2.543 | 10.36× |
| 100 | 10 | 0.536 | 5.136 | 9.59× |
| 1,000 | 1 | 0.392 | 3.664 | 9.35× |
| 1,000 | 5 | 0.767 | 20.382 | 26.56× |
| 1,000 | 10 | 1.592 | 36.274 | 22.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
