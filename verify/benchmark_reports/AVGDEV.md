# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.02M | 0.017 | 58.77M | 0.045 | 2.50× | 2.67× |
| 10,000 | 0.171 | 58.65M | 0.164 | 60.90M | 0.171 | 1.00× | 1.04× |
| 100,000 | 1.652 | 60.55M | 1.674 | 59.74M | 1.478 | 0.89× | 0.88× |
| 1,000,000 | 17.759 | 56.31M | 17.013 | 58.78M | 15.026 | 0.85× | 0.88× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.694 ms**; native kernel **1.625 ms**; TA-Lib 1.527 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.175 | 5.70M | 1480.234 | 8441.92× | 172.03× |
| 100,000 | 10 | 1.086 | 0.734 | 13.62M | 1470.569 | 2002.47× | 40.81× |
| 100,000 | 1,000 | 21.689 | 24.497 | 40.82M | 1484.223 | 60.59× | 1.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 51.74M | 49.52M | 1.00× | 2.91M | 3.20M | 1.00× | 54.19M |
| 2 | 92.47M | 106.46M | 2.15× | 2.83M | 3.49M | 1.09× | 56.38M |
| 4 | 173.02M | 175.27M | 3.54× | 2.51M | 2.78M | 0.87× | 58.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
