# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.31M | 0.020 | 50.89M | 0.056 | 2.71× | 2.86× |
| 10,000 | 0.227 | 44.04M | 0.176 | 56.71M | 0.116 | 0.51× | 0.66× |
| 100,000 | 1.781 | 56.15M | 1.699 | 58.87M | 0.730 | 0.41× | 0.43× |
| 1,000,000 | 30.569 | 32.71M | 17.310 | 57.77M | 15.882 | 0.52× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.094 ms**; native kernel **1.734 ms**; TA-Lib 0.729 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.322 | 0.265 | 3.77M | 728.697 | 2749.16× | 180.47× |
| 100,000 | 10 | 1.665 | 1.405 | 7.12M | 734.285 | 522.69× | 33.76× |
| 100,000 | 1,000 | 105.088 | 90.390 | 11.06M | 730.326 | 8.08× | 0.63× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.54M | 55.58M | 1.00× | 1.79M | 1.75M | 1.00× | 98.48M |
| 2 | 87.73M | 105.50M | 1.90× | 1.59M | 1.51M | 0.86× | 96.17M |
| 4 | 127.43M | 164.90M | 2.97× | 1.31M | 1.24M | 0.71× | 98.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
