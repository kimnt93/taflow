# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.53M | 0.005 | 221.62M | 0.027 | 4.90× | 6.09× |
| 10,000 | 0.026 | 379.69M | 0.024 | 417.30M | 0.041 | 1.57× | 1.73× |
| 100,000 | 0.236 | 424.42M | 0.218 | 459.21M | 0.173 | 0.73× | 0.79× |
| 1,000,000 | 2.831 | 353.22M | 2.253 | 443.81M | 1.638 | 0.58× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.125 | 1.15× |
| 1 | 5 | 0.307 | 0.440 | 1.43× |
| 1 | 10 | 0.472 | 0.920 | 1.95× |
| 10 | 1 | 0.053 | 0.096 | 1.80× |
| 10 | 5 | 0.249 | 0.435 | 1.75× |
| 10 | 10 | 0.494 | 0.919 | 1.86× |
| 100 | 1 | 0.055 | 0.084 | 1.52× |
| 100 | 5 | 0.265 | 0.439 | 1.66× |
| 100 | 10 | 0.492 | 0.882 | 1.79× |
| 1,000 | 1 | 0.049 | 0.100 | 2.05× |
| 1,000 | 5 | 0.226 | 0.462 | 2.04× |
| 1,000 | 10 | 0.507 | 0.906 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
