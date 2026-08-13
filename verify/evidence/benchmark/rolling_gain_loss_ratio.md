# RollingGainLossRatio benchmark (`GainLossRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.179 | 5.59M | 0.175 | 5.71M | 0.154 | 0.86× | 0.88× |
| 10,000 | 1.577 | 6.34M | 1.579 | 6.33M | 0.539 | 0.34× | 0.34× |
| 100,000 | 15.036 | 6.65M | 15.366 | 6.51M | 4.196 | 0.28× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.191 | 0.197 | 1.03× |
| 1 | 5 | 0.339 | 0.992 | 2.93× |
| 1 | 10 | 0.595 | 2.127 | 3.57× |
| 10 | 1 | 0.068 | 0.196 | 2.90× |
| 10 | 5 | 0.294 | 0.939 | 3.20× |
| 10 | 10 | 0.612 | 2.096 | 3.43× |
| 100 | 1 | 0.086 | 0.185 | 2.15× |
| 100 | 5 | 0.307 | 0.958 | 3.12× |
| 100 | 10 | 0.619 | 2.106 | 3.40× |
| 1,000 | 1 | 0.236 | 0.237 | 1.00× |
| 1,000 | 5 | 0.423 | 1.138 | 2.69× |
| 1,000 | 10 | 0.729 | 2.532 | 3.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
