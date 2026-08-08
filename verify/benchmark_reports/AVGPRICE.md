# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 248.21M | 0.002 | 523.58M | 0.030 | 7.42× | 15.64× |
| 10,000 | 0.012 | 851.11M | 0.007 | 1.35G | 0.035 | 2.97× | 4.71× |
| 100,000 | 0.091 | 1.10G | 0.060 | 1.66G | 0.092 | 1.02× | 1.53× |
| 1,000,000 | 2.111 | 473.73M | 1.567 | 638.14M | 1.842 | 0.87× | 1.18× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.086 ms**; native kernel **0.058 ms**; TA-Lib 0.092 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.336 | 0.270 | 3.70M | 95.147 | 351.86× | 104.77× |
| 100,000 | 10 | 2.519 | 1.198 | 8.35M | 95.003 | 79.33× | 22.64× |
| 100,000 | 1,000 | 6.460 | 2.826 | 353.82M | 92.087 | 32.58× | 10.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 302.36M | 526.12M | 1.00× | 2.59M | 2.55M | 1.00× | 499.93M |
| 2 | 779.25M | 902.11M | 1.71× | 2.20M | 2.86M | 1.12× | 480.23M |
| 4 | 758.06M | 1.57G | 2.99× | 2.21M | 2.40M | 0.94× | 466.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
