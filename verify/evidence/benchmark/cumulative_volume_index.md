# CumulativeVolumeIndex benchmark (`CumulativeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.68M | 0.037 | 26.98M | 4.104 | 109.52× | 110.72× |
| 10,000 | 0.257 | 38.90M | 0.245 | 40.86M | 40.369 | 157.05× | 164.96× |
| 100,000 | 2.362 | 42.34M | 2.337 | 42.80M | 401.571 | 170.03× | 171.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.229 | 1.93× |
| 1 | 5 | 0.498 | 1.307 | 2.62× |
| 1 | 10 | 0.600 | 2.080 | 3.47× |
| 10 | 1 | 0.072 | 0.238 | 3.31× |
| 10 | 5 | 0.295 | 1.140 | 3.86× |
| 10 | 10 | 0.603 | 2.582 | 4.28× |
| 100 | 1 | 0.071 | 0.610 | 8.58× |
| 100 | 5 | 0.310 | 3.081 | 9.92× |
| 100 | 10 | 0.648 | 6.426 | 9.92× |
| 1,000 | 1 | 0.100 | 4.375 | 43.73× |
| 1,000 | 5 | 0.355 | 34.707 | 97.73× |
| 1,000 | 10 | 1.545 | 55.603 | 35.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
