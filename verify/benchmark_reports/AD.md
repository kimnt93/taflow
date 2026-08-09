# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.10M | 0.008 | 132.06M | 0.030 | 2.91× | 3.96× |
| 10,000 | 0.038 | 265.01M | 0.034 | 291.47M | 0.041 | 1.08× | 1.19× |
| 100,000 | 0.321 | 311.84M | 0.291 | 344.08M | 0.152 | 0.47× | 0.52× |
| 1,000,000 | 3.695 | 270.67M | 3.242 | 308.48M | 1.827 | 0.49× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.146 | 1.89× |
| 1 | 5 | 0.280 | 0.489 | 1.75× |
| 1 | 10 | 0.529 | 0.975 | 1.84× |
| 10 | 1 | 0.051 | 0.085 | 1.68× |
| 10 | 5 | 0.253 | 0.440 | 1.74× |
| 10 | 10 | 0.481 | 0.948 | 1.97× |
| 100 | 1 | 0.050 | 0.092 | 1.83× |
| 100 | 5 | 0.240 | 0.432 | 1.80× |
| 100 | 10 | 0.523 | 0.922 | 1.76× |
| 1,000 | 1 | 0.059 | 0.092 | 1.55× |
| 1,000 | 5 | 0.272 | 0.489 | 1.80× |
| 1,000 | 10 | 0.562 | 0.937 | 1.67× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.413 | 0.262 | 3.81M | 141.276 | 538.27× | 98.59× |
| 100,000 | 10 | 2.346 | 1.120 | 8.93M | 141.228 | 126.12× | 23.11× |
| 100,000 | 1,000 | 7.357 | 4.918 | 203.35M | 149.826 | 30.47× | 5.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 166.27M | 232.24M | 1.00× | 1.88M | 2.51M | 1.00× | 335.93M |
| 5 | 427.75M | 938.46M | 4.04× | 1.98M | 2.63M | 1.05× | 398.78M |
| 10 | 476.81M | 1.02G | 4.40× | 1.99M | 2.45M | 0.98× | 371.86M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
