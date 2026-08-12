# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.76M | 0.013 | 74.87M | 0.034 | 2.11× | 2.56× |
| 10,000 | 0.123 | 81.63M | 0.122 | 81.81M | 0.112 | 0.91× | 0.92× |
| 100,000 | 1.216 | 82.22M | 1.281 | 78.07M | 0.870 | 0.72× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.106 | 1.32× |
| 1 | 5 | 0.384 | 0.467 | 1.21× |
| 1 | 10 | 0.528 | 1.007 | 1.91× |
| 10 | 1 | 0.066 | 0.102 | 1.54× |
| 10 | 5 | 0.258 | 0.461 | 1.79× |
| 10 | 10 | 0.513 | 0.931 | 1.81× |
| 100 | 1 | 0.057 | 0.096 | 1.68× |
| 100 | 5 | 0.304 | 0.501 | 1.65× |
| 100 | 10 | 0.533 | 0.953 | 1.79× |
| 1,000 | 1 | 0.066 | 0.100 | 1.51× |
| 1,000 | 5 | 0.276 | 0.492 | 1.78× |
| 1,000 | 10 | 0.622 | 1.045 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
