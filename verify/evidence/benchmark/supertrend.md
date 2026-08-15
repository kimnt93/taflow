# Supertrend benchmark (`supertrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.25M | 0.016 | 62.51M | 1.541 | 80.51× | 96.32× |
| 10,000 | 0.169 | 59.08M | 0.154 | 64.97M | 2.818 | 16.65× | 18.31× |
| 100,000 | 1.714 | 58.34M | 1.815 | 55.09M | 11.044 | 6.44× | 6.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.273 | 2.45× |
| 1 | 5 | 0.243 | 1.122 | 4.62× |
| 1 | 10 | 0.435 | 2.334 | 5.36× |
| 10 | 1 | 0.052 | 1.933 | 37.12× |
| 10 | 5 | 0.230 | 8.819 | 38.27× |
| 10 | 10 | 0.448 | 17.138 | 38.28× |
| 100 | 1 | 0.055 | 1.650 | 30.22× |
| 100 | 5 | 0.218 | 9.128 | 41.87× |
| 100 | 10 | 0.473 | 17.173 | 36.33× |
| 1,000 | 1 | 0.073 | 1.756 | 24.13× |
| 1,000 | 5 | 0.216 | 9.327 | 43.16× |
| 1,000 | 10 | 0.432 | 18.410 | 42.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
