# ParabolicMovingAverageStop benchmark (`pmax` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 9.99M | 0.087 | 11.54M | 2.789 | 27.85× | 32.18× |
| 10,000 | 0.765 | 13.07M | 0.768 | 13.02M | 15.606 | 20.40× | 20.33× |
| 100,000 | 7.633 | 13.10M | 7.663 | 13.05M | 144.929 | 18.99× | 18.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.274 | 1.90× |
| 1 | 5 | 0.463 | 1.226 | 2.65× |
| 1 | 10 | 0.712 | 2.226 | 3.13× |
| 10 | 1 | 0.083 | 1.700 | 20.37× |
| 10 | 5 | 0.329 | 8.295 | 25.17× |
| 10 | 10 | 0.726 | 17.751 | 24.46× |
| 100 | 1 | 0.112 | 2.264 | 20.22× |
| 100 | 5 | 0.444 | 11.609 | 26.17× |
| 100 | 10 | 0.868 | 25.505 | 29.39× |
| 1,000 | 1 | 0.176 | 3.154 | 17.94× |
| 1,000 | 5 | 0.351 | 16.458 | 46.91× |
| 1,000 | 10 | 0.794 | 33.050 | 41.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
