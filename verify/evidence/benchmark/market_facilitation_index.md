# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.85M | 0.003 | 319.57M | 0.189 | 33.85× | 60.49× |
| 10,000 | 0.025 | 402.59M | 0.021 | 479.51M | 1.013 | 40.79× | 48.58× |
| 100,000 | 0.198 | 505.01M | 0.174 | 575.06M | 9.078 | 45.85× | 52.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.214 | 2.59× |
| 1 | 5 | 0.244 | 1.236 | 5.07× |
| 1 | 10 | 0.386 | 1.858 | 4.82× |
| 10 | 1 | 0.051 | 0.182 | 3.54× |
| 10 | 5 | 0.195 | 0.801 | 4.11× |
| 10 | 10 | 0.414 | 1.892 | 4.57× |
| 100 | 1 | 0.050 | 0.194 | 3.90× |
| 100 | 5 | 0.207 | 0.877 | 4.24× |
| 100 | 10 | 0.435 | 1.943 | 4.47× |
| 1,000 | 1 | 0.055 | 0.270 | 4.93× |
| 1,000 | 5 | 0.192 | 1.226 | 6.37× |
| 1,000 | 10 | 0.446 | 2.695 | 6.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
