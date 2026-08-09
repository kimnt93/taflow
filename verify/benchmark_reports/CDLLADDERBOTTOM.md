# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.41M | 0.004 | 278.30M | 0.032 | 6.03× | 8.90× |
| 10,000 | 0.057 | 176.40M | 0.052 | 192.16M | 0.080 | 1.41× | 1.54× |
| 100,000 | 0.580 | 172.36M | 0.576 | 173.53M | 0.561 | 0.97× | 0.97× |
| 1,000,000 | 6.367 | 157.07M | 6.108 | 163.72M | 5.638 | 0.89× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.584 ms**; native kernel **0.584 ms**; TA-Lib 0.554 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.325 | 0.258 | 3.87M | 559.908 | 2167.48× | 105.35× |
| 100,000 | 10 | 2.773 | 1.322 | 7.56M | 563.771 | 426.30× | 20.67× |
| 100,000 | 1,000 | 27.059 | 26.367 | 37.93M | 559.721 | 21.23× | 1.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 135.38M | 144.94M | 1.00× | 2.26M | 2.60M | 1.00× | 147.01M |
| 2 | 245.26M | 256.89M | 1.77× | 2.37M | 2.61M | 1.00× | 137.33M |
| 4 | 392.76M | 483.93M | 3.34× | 2.41M | 2.50M | 0.96× | 135.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
