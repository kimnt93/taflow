# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.76M | 0.013 | 74.36M | 0.226 | 14.89× | 16.83× |
| 10,000 | 0.105 | 95.18M | 0.101 | 98.72M | 0.861 | 8.20× | 8.50× |
| 100,000 | 1.061 | 94.25M | 0.945 | 105.81M | 8.086 | 7.62× | 8.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.281 | 4.49× |
| 1 | 5 | 0.338 | 1.064 | 3.15× |
| 1 | 10 | 0.520 | 2.413 | 4.64× |
| 10 | 1 | 0.067 | 0.222 | 3.32× |
| 10 | 5 | 0.252 | 1.277 | 5.07× |
| 10 | 10 | 0.543 | 2.368 | 4.36× |
| 100 | 1 | 0.055 | 0.219 | 3.98× |
| 100 | 5 | 0.243 | 1.280 | 5.26× |
| 100 | 10 | 0.530 | 2.400 | 4.53× |
| 1,000 | 1 | 0.065 | 0.293 | 4.49× |
| 1,000 | 5 | 0.241 | 1.703 | 7.06× |
| 1,000 | 10 | 0.519 | 3.163 | 6.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
