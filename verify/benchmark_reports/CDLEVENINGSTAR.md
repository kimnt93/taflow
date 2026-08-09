# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.72M | 0.005 | 197.12M | 0.037 | 5.31× | 7.23× |
| 10,000 | 0.079 | 125.87M | 0.075 | 132.91M | 0.110 | 1.38× | 1.46× |
| 100,000 | 0.854 | 117.03M | 0.840 | 119.09M | 0.834 | 0.98× | 0.99× |
| 1,000,000 | 8.662 | 115.45M | 8.651 | 115.59M | 8.242 | 0.95× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.855 ms**; native kernel **0.847 ms**; TA-Lib 0.830 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.277 | 3.61M | 837.647 | 3023.51× | 117.50× |
| 100,000 | 10 | 2.539 | 1.405 | 7.12M | 834.525 | 593.77× | 22.88× |
| 100,000 | 1,000 | 31.094 | 30.536 | 32.75M | 847.002 | 27.74× | 1.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.54M | 94.83M | 1.00× | 2.24M | 2.36M | 1.00× | 97.88M |
| 2 | 185.75M | 189.59M | 2.00× | 2.46M | 2.41M | 1.02× | 100.90M |
| 4 | 334.01M | 365.25M | 3.85× | 2.22M | 2.51M | 1.06× | 100.56M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
