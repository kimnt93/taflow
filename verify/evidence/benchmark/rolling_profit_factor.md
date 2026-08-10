# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.61M | 0.025 | 39.51M | 0.158 | 5.79× | 6.25× |
| 10,000 | 0.226 | 44.17M | 0.245 | 40.86M | 0.643 | 2.84× | 2.63× |
| 100,000 | 2.427 | 41.20M | 2.241 | 44.63M | 5.154 | 2.12× | 2.30× |
| 1,000,000 | 22.620 | 44.21M | 22.307 | 44.83M | 48.487 | 2.14× | 2.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.266 | 2.13× |
| 1 | 5 | 0.373 | 0.975 | 2.62× |
| 1 | 10 | 0.483 | 2.085 | 4.32× |
| 10 | 1 | 0.047 | 0.194 | 4.11× |
| 10 | 5 | 0.222 | 0.938 | 4.23× |
| 10 | 10 | 0.474 | 2.096 | 4.43× |
| 100 | 1 | 0.056 | 0.208 | 3.73× |
| 100 | 5 | 0.221 | 0.948 | 4.29× |
| 100 | 10 | 0.506 | 2.079 | 4.11× |
| 1,000 | 1 | 0.076 | 0.249 | 3.28× |
| 1,000 | 5 | 0.256 | 1.226 | 4.79× |
| 1,000 | 10 | 0.503 | 2.658 | 5.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
