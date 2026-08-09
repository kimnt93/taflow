# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.61M | 0.006 | 154.15M | 0.030 | 3.98× | 4.62× |
| 10,000 | 0.050 | 201.84M | 0.048 | 207.04M | 0.070 | 1.41× | 1.45× |
| 100,000 | 0.460 | 217.42M | 0.438 | 228.48M | 0.448 | 0.97× | 1.02× |
| 1,000,000 | 4.972 | 201.13M | 4.440 | 225.25M | 4.252 | 0.86× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.104 | 1.24× |
| 1 | 5 | 0.388 | 0.528 | 1.36× |
| 1 | 10 | 0.600 | 1.273 | 2.12× |
| 10 | 1 | 0.055 | 0.086 | 1.58× |
| 10 | 5 | 0.229 | 0.466 | 2.03× |
| 10 | 10 | 0.489 | 0.953 | 1.95× |
| 100 | 1 | 0.047 | 0.088 | 1.88× |
| 100 | 5 | 0.248 | 0.427 | 1.72× |
| 100 | 10 | 0.503 | 0.942 | 1.87× |
| 1,000 | 1 | 0.056 | 0.098 | 1.74× |
| 1,000 | 5 | 0.244 | 0.443 | 1.82× |
| 1,000 | 10 | 0.510 | 0.944 | 1.85× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.331 | 0.179 | 5.59M | 435.187 | 2430.85× | 133.84× |
| 100,000 | 10 | 0.960 | 0.516 | 19.39M | 439.882 | 852.98× | 45.96× |
| 100,000 | 1,000 | 6.913 | 5.710 | 175.12M | 455.095 | 79.70× | 5.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 182.80M | 147.24M | 1.00× | 2.69M | 3.55M | 1.00× | 159.32M |
| 5 | 268.96M | 492.88M | 3.35× | 2.15M | 2.82M | 0.80× | 178.74M |
| 10 | 555.07M | 790.32M | 5.37× | 2.14M | 2.68M | 0.75× | 182.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
