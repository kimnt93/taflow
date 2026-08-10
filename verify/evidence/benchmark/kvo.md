# KlingerVolumeOscillator benchmark (`KVO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.37M | 0.025 | 39.90M | 0.324 | 11.46× | 12.93× |
| 10,000 | 0.208 | 48.18M | 0.199 | 50.23M | 1.577 | 7.60× | 7.92× |
| 100,000 | 1.941 | 51.51M | 2.009 | 49.78M | 13.713 | 7.06× | 6.83× |
| 1,000,000 | 20.178 | 49.56M | 18.766 | 53.29M | 138.465 | 6.86× | 7.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.328 | 3.29× |
| 1 | 5 | 0.365 | 1.515 | 4.15× |
| 1 | 10 | 0.597 | 3.780 | 6.33× |
| 10 | 1 | 0.064 | 0.265 | 4.13× |
| 10 | 5 | 0.259 | 1.585 | 6.13× |
| 10 | 10 | 0.577 | 2.928 | 5.07× |
| 100 | 1 | 0.059 | 0.281 | 4.73× |
| 100 | 5 | 0.292 | 1.640 | 5.61× |
| 100 | 10 | 0.627 | 3.287 | 5.24× |
| 1,000 | 1 | 0.078 | 0.409 | 5.25× |
| 1,000 | 5 | 0.303 | 2.383 | 7.87× |
| 1,000 | 10 | 0.657 | 4.508 | 6.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
