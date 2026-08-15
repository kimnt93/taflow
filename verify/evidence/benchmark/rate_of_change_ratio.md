# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 439.75M | 0.001 | 724.55M | 0.032 | 14.13× | 23.27× |
| 10,000 | 0.008 | 1.18G | 0.006 | 1.71G | 0.042 | 4.90× | 7.12× |
| 100,000 | 0.075 | 1.33G | 0.048 | 2.09G | 0.127 | 1.69× | 2.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.106 | 1.68× |
| 1 | 5 | 0.295 | 0.452 | 1.53× |
| 1 | 10 | 0.393 | 0.968 | 2.47× |
| 10 | 1 | 0.043 | 0.093 | 2.18× |
| 10 | 5 | 0.183 | 0.442 | 2.41× |
| 10 | 10 | 0.356 | 0.924 | 2.59× |
| 100 | 1 | 0.040 | 0.088 | 2.19× |
| 100 | 5 | 0.207 | 0.508 | 2.46× |
| 100 | 10 | 0.393 | 0.922 | 2.35× |
| 1,000 | 1 | 0.043 | 0.086 | 1.99× |
| 1,000 | 5 | 0.196 | 0.449 | 2.29× |
| 1,000 | 10 | 0.425 | 0.974 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
