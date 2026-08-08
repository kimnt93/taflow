# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 195.38M | 0.004 | 280.50M | 0.030 | 5.86× | 8.42× |
| 10,000 | 0.072 | 138.08M | 0.067 | 149.00M | 0.083 | 1.15× | 1.24× |
| 100,000 | 0.805 | 124.20M | 0.775 | 128.98M | 0.593 | 0.74× | 0.76× |
| 1,000,000 | 8.436 | 118.55M | 8.284 | 120.71M | 5.852 | 0.69× | 0.71× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.803 ms**; native kernel **0.805 ms**; TA-Lib 0.587 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.290 | 3.45M | 579.228 | 1995.60× | 95.39× |
| 100,000 | 10 | 2.634 | 1.335 | 7.49M | 570.946 | 427.55× | 19.80× |
| 100,000 | 1,000 | 23.239 | 21.750 | 45.98M | 619.391 | 28.48× | 1.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 100.94M | 95.04M | 1.00× | 2.22M | 2.39M | 1.00× | 124.94M |
| 2 | 201.55M | 211.23M | 2.22× | 2.33M | 2.70M | 1.13× | 136.13M |
| 4 | 374.66M | 374.90M | 3.94× | 2.11M | 2.53M | 1.06× | 124.85M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
