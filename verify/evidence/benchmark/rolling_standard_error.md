# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.45M | 0.034 | 29.56M | 0.206 | 5.85× | 6.08× |
| 10,000 | 0.318 | 31.47M | 0.314 | 31.81M | 0.729 | 2.29× | 2.32× |
| 100,000 | 3.168 | 31.57M | 3.216 | 31.09M | 6.055 | 1.91× | 1.88× |
| 1,000,000 | 31.370 | 31.88M | 31.388 | 31.86M | 57.547 | 1.83× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.270 | 3.58× |
| 1 | 5 | 0.243 | 1.320 | 5.44× |
| 1 | 10 | 0.523 | 2.483 | 4.75× |
| 10 | 1 | 0.053 | 0.210 | 3.93× |
| 10 | 5 | 0.277 | 1.423 | 5.13× |
| 10 | 10 | 0.546 | 2.543 | 4.66× |
| 100 | 1 | 0.061 | 0.234 | 3.84× |
| 100 | 5 | 0.283 | 1.322 | 4.67× |
| 100 | 10 | 0.553 | 2.609 | 4.71× |
| 1,000 | 1 | 0.085 | 0.276 | 3.23× |
| 1,000 | 5 | 0.288 | 1.768 | 6.15× |
| 1,000 | 10 | 0.592 | 3.169 | 5.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
