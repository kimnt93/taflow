# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.23M | 0.010 | 96.13M | 0.040 | 3.49× | 3.80× |
| 10,000 | 0.098 | 102.20M | 0.096 | 103.70M | 0.114 | 1.17× | 1.18× |
| 100,000 | 0.951 | 105.20M | 0.920 | 108.68M | 0.879 | 0.93× | 0.96× |
| 1,000,000 | 9.680 | 103.31M | 9.138 | 109.43M | 9.220 | 0.95× | 1.01× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.945 ms**; native kernel **0.926 ms**; TA-Lib 0.880 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.157 | 6.37M | 895.667 | 5702.32× | 200.10× |
| 100,000 | 10 | 0.974 | 0.590 | 16.95M | 873.274 | 1480.59× | 52.17× |
| 100,000 | 1,000 | 11.799 | 10.764 | 92.90M | 891.637 | 82.83× | 3.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 95.08M | 96.61M | 1.00× | 2.95M | 3.74M | 1.00× | 91.79M |
| 2 | 175.59M | 194.14M | 2.01× | 3.02M | 4.20M | 1.12× | 93.05M |
| 4 | 262.36M | 349.87M | 3.62× | 2.71M | 3.07M | 0.82× | 89.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
