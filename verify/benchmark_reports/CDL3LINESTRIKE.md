# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.13M | 0.005 | 201.12M | 0.033 | 4.92× | 6.55× |
| 10,000 | 0.067 | 148.94M | 0.062 | 162.25M | 0.102 | 1.51× | 1.65× |
| 100,000 | 0.806 | 124.12M | 0.784 | 127.53M | 0.775 | 0.96× | 0.99× |
| 1,000,000 | 8.287 | 120.67M | 8.189 | 122.11M | 7.729 | 0.93× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.798 ms**; native kernel **0.792 ms**; TA-Lib 0.771 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.336 | 0.275 | 3.64M | 767.500 | 2791.81× | 94.93× |
| 100,000 | 10 | 2.598 | 1.475 | 6.78M | 768.528 | 521.19× | 18.21× |
| 100,000 | 1,000 | 43.657 | 35.672 | 28.03M | 792.707 | 22.22× | 0.88× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.40M | 100.53M | 1.00× | 2.07M | 2.31M | 1.00× | 107.87M |
| 2 | 185.23M | 203.01M | 2.02× | 2.16M | 2.37M | 1.02× | 103.82M |
| 4 | 332.41M | 344.49M | 3.43× | 2.12M | 2.31M | 1.00× | 108.57M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
