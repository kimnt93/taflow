# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 210.35M | 0.004 | 263.72M | 0.033 | 6.95× | 8.72× |
| 10,000 | 0.021 | 472.16M | 0.019 | 536.51M | 0.041 | 1.95× | 2.22× |
| 100,000 | 0.184 | 543.88M | 0.161 | 619.67M | 0.129 | 0.70× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.132 | 1.67× |
| 1 | 5 | 0.366 | 0.491 | 1.34× |
| 1 | 10 | 0.473 | 0.992 | 2.10× |
| 10 | 1 | 0.056 | 0.089 | 1.60× |
| 10 | 5 | 0.252 | 0.496 | 1.96× |
| 10 | 10 | 0.486 | 0.932 | 1.92× |
| 100 | 1 | 0.055 | 0.093 | 1.70× |
| 100 | 5 | 0.227 | 0.433 | 1.91× |
| 100 | 10 | 0.499 | 0.938 | 1.88× |
| 1,000 | 1 | 0.049 | 0.089 | 1.83× |
| 1,000 | 5 | 0.236 | 0.440 | 1.86× |
| 1,000 | 10 | 0.485 | 0.960 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
