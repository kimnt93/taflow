# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.15M | 0.009 | 112.35M | 0.259 | 21.24× | 29.04× |
| 10,000 | 0.048 | 210.08M | 0.038 | 261.75M | 1.634 | 34.32× | 42.77× |
| 100,000 | 0.405 | 246.96M | 0.594 | 168.24M | 25.327 | 62.55× | 42.61× |
| 1,000,000 | 4.045 | 247.24M | 3.515 | 284.49M | 143.618 | 35.51× | 40.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.235 | 2.60× |
| 1 | 5 | 0.311 | 1.161 | 3.74× |
| 1 | 10 | 0.572 | 1.683 | 2.94× |
| 10 | 1 | 0.055 | 0.168 | 3.05× |
| 10 | 5 | 0.288 | 1.149 | 4.00× |
| 10 | 10 | 0.535 | 1.809 | 3.38× |
| 100 | 1 | 0.064 | 0.190 | 2.99× |
| 100 | 5 | 0.279 | 1.198 | 4.30× |
| 100 | 10 | 0.539 | 1.920 | 3.56× |
| 1,000 | 1 | 0.064 | 0.300 | 4.67× |
| 1,000 | 5 | 0.296 | 1.809 | 6.12× |
| 1,000 | 10 | 0.564 | 3.033 | 5.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
