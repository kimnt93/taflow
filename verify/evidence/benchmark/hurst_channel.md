# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.03M | 0.049 | 20.40M | 0.675 | 14.86× | 13.76× |
| 10,000 | 0.402 | 24.87M | 0.380 | 26.35M | 4.354 | 10.83× | 11.47× |
| 100,000 | 4.965 | 20.14M | 4.018 | 24.89M | 50.594 | 10.19× | 12.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.390 | 2.33× |
| 1 | 5 | 0.309 | 1.456 | 4.70× |
| 1 | 10 | 0.555 | 2.709 | 4.88× |
| 10 | 1 | 0.061 | 0.295 | 4.87× |
| 10 | 5 | 0.283 | 1.554 | 5.50× |
| 10 | 10 | 0.576 | 3.012 | 5.23× |
| 100 | 1 | 0.061 | 0.315 | 5.12× |
| 100 | 5 | 0.281 | 1.719 | 6.13× |
| 100 | 10 | 0.597 | 3.284 | 5.50× |
| 1,000 | 1 | 0.095 | 0.965 | 10.11× |
| 1,000 | 5 | 0.323 | 4.228 | 13.08× |
| 1,000 | 10 | 0.605 | 14.767 | 24.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
