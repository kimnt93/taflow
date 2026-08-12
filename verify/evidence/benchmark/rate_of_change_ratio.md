# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.35M | 0.004 | 256.57M | 0.033 | 3.86× | 8.44× |
| 10,000 | 0.021 | 470.72M | 0.018 | 554.45M | 0.040 | 1.88× | 2.22× |
| 100,000 | 0.182 | 550.05M | 0.157 | 637.41M | 0.125 | 0.69× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.191 | 0.132 | 0.69× |
| 1 | 5 | 0.262 | 0.493 | 1.88× |
| 1 | 10 | 0.491 | 0.930 | 1.89× |
| 10 | 1 | 0.048 | 0.090 | 1.88× |
| 10 | 5 | 0.214 | 0.455 | 2.13× |
| 10 | 10 | 0.503 | 0.941 | 1.87× |
| 100 | 1 | 0.052 | 0.089 | 1.71× |
| 100 | 5 | 0.202 | 0.433 | 2.14× |
| 100 | 10 | 0.487 | 0.957 | 1.97× |
| 1,000 | 1 | 0.059 | 0.105 | 1.77× |
| 1,000 | 5 | 0.291 | 0.498 | 1.71× |
| 1,000 | 10 | 0.511 | 0.928 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
