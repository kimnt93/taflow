# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.72M | 0.005 | 220.39M | 0.039 | 5.65× | 8.49× |
| 10,000 | 0.082 | 121.64M | 0.079 | 127.23M | 0.133 | 1.62× | 1.69× |
| 100,000 | 0.999 | 100.06M | 0.992 | 100.83M | 1.061 | 1.06× | 1.07× |
| 1,000,000 | 10.741 | 93.10M | 10.511 | 95.14M | 11.021 | 1.03× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.010 ms**; native kernel **1.002 ms**; TA-Lib 1.089 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.381 | 0.284 | 3.52M | 1103.186 | 3885.54× | 96.98× |
| 100,000 | 10 | 3.359 | 1.902 | 5.26M | 1083.448 | 569.64× | 14.66× |
| 100,000 | 1,000 | 31.306 | 31.043 | 32.21M | 1073.829 | 34.59× | 1.08× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.37M | 88.71M | 1.00× | 2.19M | 2.38M | 1.00× | 79.89M |
| 2 | 161.05M | 167.79M | 1.89× | 2.42M | 2.45M | 1.03× | 81.73M |
| 4 | 311.42M | 323.93M | 3.65× | 2.21M | 2.24M | 0.94× | 81.52M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
