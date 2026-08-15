# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.351 | 2.85M | 0.350 | 2.85M | 3.175 | 9.05× | 9.06× |
| 10,000 | 3.453 | 2.90M | 3.502 | 2.86M | 31.740 | 9.19× | 9.06× |
| 100,000 | 36.479 | 2.74M | 36.252 | 2.76M | 315.926 | 8.66× | 8.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.285 | 3.09× |
| 1 | 5 | 0.276 | 1.334 | 4.83× |
| 1 | 10 | 0.431 | 2.579 | 5.98× |
| 10 | 1 | 0.052 | 0.249 | 4.78× |
| 10 | 5 | 0.190 | 1.358 | 7.15× |
| 10 | 10 | 0.464 | 2.680 | 5.78× |
| 100 | 1 | 0.079 | 0.483 | 6.13× |
| 100 | 5 | 0.223 | 2.677 | 12.01× |
| 100 | 10 | 0.451 | 5.103 | 11.31× |
| 1,000 | 1 | 0.403 | 3.790 | 9.39× |
| 1,000 | 5 | 0.592 | 24.481 | 41.33× |
| 1,000 | 10 | 1.164 | 37.972 | 32.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
