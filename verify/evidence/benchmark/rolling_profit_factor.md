# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.31M | 0.025 | 39.49M | 0.190 | 7.28× | 7.50× |
| 10,000 | 0.234 | 42.65M | 0.231 | 43.30M | 0.652 | 2.78× | 2.82× |
| 100,000 | 2.289 | 43.69M | 2.398 | 41.70M | 5.072 | 2.22× | 2.11× |
| 1,000,000 | 24.044 | 41.59M | 23.545 | 42.47M | 51.436 | 2.14× | 2.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.240 | 2.59× |
| 1 | 5 | 0.272 | 1.011 | 3.72× |
| 1 | 10 | 0.469 | 2.057 | 4.38× |
| 10 | 1 | 0.050 | 0.189 | 3.82× |
| 10 | 5 | 0.236 | 0.964 | 4.08× |
| 10 | 10 | 0.465 | 2.165 | 4.66× |
| 100 | 1 | 0.055 | 0.203 | 3.66× |
| 100 | 5 | 0.228 | 0.997 | 4.36× |
| 100 | 10 | 0.494 | 2.135 | 4.32× |
| 1,000 | 1 | 0.074 | 0.250 | 3.38× |
| 1,000 | 5 | 0.239 | 1.210 | 5.06× |
| 1,000 | 10 | 0.501 | 2.676 | 5.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
