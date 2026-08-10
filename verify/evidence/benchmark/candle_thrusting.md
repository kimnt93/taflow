# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.81M | 0.017 | 60.48M | 0.032 | 1.72× | 1.93× |
| 10,000 | 0.149 | 66.91M | 0.160 | 62.41M | 0.116 | 0.77× | 0.72× |
| 100,000 | 1.348 | 74.20M | 1.499 | 66.73M | 0.908 | 0.67× | 0.61× |
| 1,000,000 | 13.907 | 71.91M | 15.895 | 62.91M | 8.986 | 0.65× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.129 | 1.08× |
| 1 | 5 | 0.357 | 0.472 | 1.32× |
| 1 | 10 | 0.541 | 0.918 | 1.70× |
| 10 | 1 | 0.063 | 0.090 | 1.42× |
| 10 | 5 | 0.250 | 0.428 | 1.71× |
| 10 | 10 | 0.533 | 0.911 | 1.71× |
| 100 | 1 | 0.055 | 0.090 | 1.63× |
| 100 | 5 | 0.267 | 0.448 | 1.68× |
| 100 | 10 | 0.523 | 0.949 | 1.81× |
| 1,000 | 1 | 0.073 | 0.107 | 1.47× |
| 1,000 | 5 | 0.257 | 0.508 | 1.98× |
| 1,000 | 10 | 0.564 | 0.994 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
