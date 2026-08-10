# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.15M | 0.020 | 48.98M | 0.202 | 13.35× | 9.88× |
| 10,000 | 0.121 | 82.33M | 0.109 | 91.96M | 0.548 | 4.52× | 5.04× |
| 100,000 | 1.185 | 84.41M | 1.054 | 94.90M | 4.102 | 3.46× | 3.89× |
| 1,000,000 | 12.095 | 82.68M | 10.473 | 95.48M | 40.792 | 3.37× | 3.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.313 | 3.86× |
| 1 | 5 | 0.303 | 1.224 | 4.04× |
| 1 | 10 | 0.473 | 2.441 | 5.16× |
| 10 | 1 | 0.049 | 0.231 | 4.67× |
| 10 | 5 | 0.222 | 1.339 | 6.04× |
| 10 | 10 | 0.481 | 2.593 | 5.39× |
| 100 | 1 | 0.052 | 0.243 | 4.63× |
| 100 | 5 | 0.242 | 1.385 | 5.73× |
| 100 | 10 | 0.497 | 2.607 | 5.25× |
| 1,000 | 1 | 0.061 | 0.283 | 4.62× |
| 1,000 | 5 | 0.253 | 1.579 | 6.24× |
| 1,000 | 10 | 0.527 | 3.164 | 6.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
