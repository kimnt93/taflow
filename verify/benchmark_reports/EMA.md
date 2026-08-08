# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.28M | 0.004 | 234.36M | 0.035 | 6.45× | 8.25× |
| 10,000 | 0.040 | 247.32M | 0.041 | 243.99M | 0.061 | 1.50× | 1.48× |
| 100,000 | 0.392 | 254.82M | 0.368 | 271.73M | 0.311 | 0.79× | 0.84× |
| 1,000,000 | 4.919 | 203.31M | 4.371 | 228.76M | 2.829 | 0.58× | 0.65× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.366 ms**; native kernel **0.349 ms**; TA-Lib 0.309 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.245 | 0.151 | 6.63M | 311.463 | 2065.23× | 203.83× |
| 100,000 | 10 | 1.033 | 0.527 | 18.99M | 300.463 | 570.59× | 57.15× |
| 100,000 | 1,000 | 6.531 | 5.070 | 197.25M | 305.159 | 60.19× | 6.61× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 173.39M | 200.47M | 1.00× | 2.64M | 3.07M | 1.00× | 232.82M |
| 2 | 341.35M | 362.78M | 1.81× | 3.37M | 3.96M | 1.29× | 233.46M |
| 4 | 381.71M | 619.03M | 3.09× | 3.14M | 3.27M | 1.07× | 244.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
