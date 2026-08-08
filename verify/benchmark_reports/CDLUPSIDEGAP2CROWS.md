# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.07M | 0.004 | 230.55M | 0.033 | 5.31× | 7.69× |
| 10,000 | 0.092 | 108.62M | 0.085 | 118.18M | 0.122 | 1.33× | 1.45× |
| 100,000 | 0.878 | 113.94M | 0.874 | 114.37M | 0.999 | 1.14× | 1.14× |
| 1,000,000 | 9.743 | 102.63M | 9.201 | 108.68M | 9.767 | 1.00× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.880 ms**; native kernel **0.861 ms**; TA-Lib 1.006 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.408 | 0.287 | 3.48M | 1010.642 | 3515.49× | 100.52× |
| 100,000 | 10 | 2.710 | 1.477 | 6.77M | 1045.856 | 708.21× | 20.42× |
| 100,000 | 1,000 | 29.779 | 27.296 | 36.64M | 1041.525 | 38.16× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 94.79M | 98.38M | 1.00× | 1.51M | 2.32M | 1.00× | 82.13M |
| 2 | 177.59M | 167.24M | 1.70× | 2.11M | 2.51M | 1.08× | 88.89M |
| 4 | 293.70M | 361.53M | 3.67× | 2.26M | 2.47M | 1.06× | 87.71M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
