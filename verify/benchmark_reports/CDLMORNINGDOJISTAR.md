# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.43M | 0.005 | 213.71M | 0.039 | 6.04× | 8.31× |
| 10,000 | 0.080 | 124.43M | 0.080 | 125.48M | 0.113 | 1.41× | 1.42× |
| 100,000 | 0.856 | 116.77M | 0.838 | 119.31M | 0.833 | 0.97× | 0.99× |
| 1,000,000 | 9.175 | 108.99M | 8.827 | 113.29M | 8.297 | 0.90× | 0.94× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.871 ms**; native kernel **0.838 ms**; TA-Lib 0.825 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.333 | 0.260 | 3.84M | 828.398 | 3181.16× | 124.65× |
| 100,000 | 10 | 2.519 | 1.324 | 7.55M | 839.990 | 634.25× | 24.23× |
| 100,000 | 1,000 | 30.802 | 26.925 | 37.14M | 842.304 | 31.28× | 1.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.92M | 96.33M | 1.00× | 2.04M | 2.38M | 1.00× | 102.23M |
| 2 | 189.69M | 200.46M | 2.08× | 2.46M | 2.78M | 1.17× | 96.61M |
| 4 | 327.31M | 372.94M | 3.87× | 2.30M | 2.45M | 1.03× | 101.01M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
