# NewHighsNewLows benchmark (`NewHighsNewLows` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.46M | 0.004 | 280.62M | 8.216 | 1737.38× | 2305.67× |
| 10,000 | 0.027 | 369.34M | 0.025 | 407.65M | 84.136 | 3107.48× | 3429.76× |
| 100,000 | 0.255 | 392.09M | 0.220 | 454.14M | 824.967 | 3234.59× | 3746.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.279 | 1.66× |
| 1 | 5 | 0.230 | 1.400 | 6.08× |
| 1 | 10 | 0.403 | 2.093 | 5.20× |
| 10 | 1 | 0.048 | 0.279 | 5.86× |
| 10 | 5 | 0.184 | 1.690 | 9.20× |
| 10 | 10 | 0.403 | 2.901 | 7.20× |
| 100 | 1 | 0.048 | 1.047 | 21.66× |
| 100 | 5 | 0.193 | 5.589 | 28.95× |
| 100 | 10 | 0.477 | 10.907 | 22.86× |
| 1,000 | 1 | 0.053 | 8.472 | 158.72× |
| 1,000 | 5 | 0.253 | 44.100 | 174.26× |
| 1,000 | 10 | 0.485 | 91.551 | 188.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
