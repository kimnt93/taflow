# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.46M | 0.019 | 53.54M | 0.278 | 12.89× | 14.86× |
| 10,000 | 0.179 | 55.90M | 0.183 | 54.55M | 1.516 | 8.47× | 8.27× |
| 100,000 | 1.813 | 55.16M | 1.754 | 57.01M | 13.962 | 7.70× | 7.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.274 | 1.51× |
| 1 | 5 | 0.365 | 1.119 | 3.06× |
| 1 | 10 | 0.430 | 2.438 | 5.67× |
| 10 | 1 | 0.047 | 0.213 | 4.56× |
| 10 | 5 | 0.191 | 1.101 | 5.77× |
| 10 | 10 | 0.427 | 2.261 | 5.29× |
| 100 | 1 | 0.049 | 0.235 | 4.81× |
| 100 | 5 | 0.219 | 1.370 | 6.25× |
| 100 | 10 | 0.423 | 2.375 | 5.61× |
| 1,000 | 1 | 0.077 | 0.374 | 4.84× |
| 1,000 | 5 | 0.218 | 1.961 | 9.01× |
| 1,000 | 10 | 0.427 | 3.737 | 8.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
