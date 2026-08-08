# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.56M | 0.004 | 246.13M | 0.033 | 5.49× | 8.21× |
| 10,000 | 0.067 | 148.38M | 0.057 | 174.45M | 0.123 | 1.82× | 2.14× |
| 100,000 | 0.844 | 118.50M | 0.853 | 117.21M | 0.960 | 1.14× | 1.13× |
| 1,000,000 | 9.135 | 109.47M | 8.943 | 111.82M | 9.508 | 1.04× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.844 ms**; native kernel **0.811 ms**; TA-Lib 0.942 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.532 | 0.284 | 3.52M | 964.452 | 3394.94× | 101.11× |
| 100,000 | 10 | 2.676 | 1.391 | 7.19M | 948.388 | 681.56× | 20.53× |
| 100,000 | 1,000 | 33.028 | 28.101 | 35.59M | 969.272 | 34.49× | 1.16× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 90.87M | 99.36M | 1.00× | 2.16M | 2.29M | 1.00× | 82.28M |
| 2 | 184.67M | 202.99M | 2.04× | 2.23M | 2.52M | 1.10× | 92.32M |
| 4 | 305.38M | 338.61M | 3.41× | 2.23M | 2.45M | 1.07× | 90.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
