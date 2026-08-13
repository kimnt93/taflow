# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.34M | 0.057 | 17.57M | 0.030 | 0.46× | 0.53× |
| 10,000 | 0.447 | 22.36M | 0.448 | 22.32M | 0.084 | 0.19× | 0.19× |
| 100,000 | 4.348 | 23.00M | 4.351 | 22.99M | 0.592 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.146 | 1.25× |
| 1 | 5 | 0.427 | 0.465 | 1.09× |
| 1 | 10 | 0.678 | 0.956 | 1.41× |
| 10 | 1 | 0.079 | 0.089 | 1.13× |
| 10 | 5 | 0.315 | 0.442 | 1.40× |
| 10 | 10 | 0.664 | 0.889 | 1.34× |
| 100 | 1 | 0.075 | 0.087 | 1.16× |
| 100 | 5 | 0.313 | 0.418 | 1.33× |
| 100 | 10 | 0.658 | 0.920 | 1.40× |
| 1,000 | 1 | 0.115 | 0.103 | 0.90× |
| 1,000 | 5 | 0.327 | 0.480 | 1.47× |
| 1,000 | 10 | 0.670 | 0.960 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
