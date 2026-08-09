# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.39M | 0.014 | 73.63M | 1.495 | 90.30× | 110.09× |
| 10,000 | 0.089 | 112.99M | 0.079 | 126.79M | 1.890 | 21.35× | 23.96× |
| 100,000 | 0.831 | 120.31M | 0.752 | 132.90M | 6.133 | 7.38× | 8.15× |
| 1,000,000 | 18.573 | 53.84M | 7.861 | 127.20M | 72.311 | 3.89× | 9.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 1.602 | 19.96× |
| 1 | 5 | 0.309 | 8.607 | 27.84× |
| 1 | 10 | 0.569 | 16.533 | 29.07× |
| 10 | 1 | 0.063 | 1.675 | 26.54× |
| 10 | 5 | 0.289 | 7.979 | 27.61× |
| 10 | 10 | 0.582 | 16.616 | 28.56× |
| 100 | 1 | 0.063 | 1.593 | 25.40× |
| 100 | 5 | 0.328 | 7.979 | 24.35× |
| 100 | 10 | 0.637 | 16.528 | 25.94× |
| 1,000 | 1 | 0.072 | 1.712 | 23.72× |
| 1,000 | 5 | 0.333 | 9.130 | 27.41× |
| 1,000 | 10 | 0.599 | 18.161 | 30.30× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.613 | 0.561 | 1.78M | 6005.718 | 10697.96× |
| 100,000 | 10 | 4.108 | 2.094 | 4.78M | 6426.829 | 3069.79× |
| 100,000 | 1,000 | 96.945 | 107.187 | 9.33M | 6030.380 | 56.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 69.03M | 92.88M | 1.00× | 1.19M | 1.01M | 1.00× | 13.69M |
| 5 | 132.48M | 217.52M | 2.34× | 976.34K | 1.04M | 1.04× | 29.05M |
| 10 | 179.15M | 367.77M | 3.96× | 1.00M | 1.08M | 1.07× | 26.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
