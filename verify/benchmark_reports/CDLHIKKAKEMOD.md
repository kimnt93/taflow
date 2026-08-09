# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.31M | 0.007 | 153.36M | 0.032 | 3.83× | 4.88× |
| 10,000 | 0.063 | 158.30M | 0.058 | 173.21M | 0.079 | 1.25× | 1.37× |
| 100,000 | 0.576 | 173.52M | 0.582 | 171.80M | 0.546 | 0.95× | 0.94× |
| 1,000,000 | 5.972 | 167.45M | 5.784 | 172.90M | 5.308 | 0.89× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.583 ms**; native kernel **0.565 ms**; TA-Lib 0.540 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.322 | 0.299 | 3.35M | 540.712 | 1809.61× | 89.83× |
| 100,000 | 10 | 2.495 | 1.309 | 7.64M | 538.498 | 411.40× | 20.97× |
| 100,000 | 1,000 | 21.809 | 18.680 | 53.53M | 544.795 | 29.17× | 1.72× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 130.45M | 128.02M | 1.00× | 2.20M | 2.38M | 1.00× | 130.01M |
| 2 | 259.02M | 287.72M | 2.25× | 2.36M | 2.58M | 1.09× | 144.27M |
| 4 | 463.34M | 533.35M | 4.17× | 2.53M | 2.66M | 1.12× | 147.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
