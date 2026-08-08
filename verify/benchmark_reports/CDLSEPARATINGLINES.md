# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.42M | 0.006 | 171.46M | 0.035 | 4.41× | 5.98× |
| 10,000 | 0.076 | 131.21M | 0.069 | 143.99M | 0.132 | 1.73× | 1.90× |
| 100,000 | 0.830 | 120.41M | 0.855 | 116.95M | 1.070 | 1.29× | 1.25× |
| 1,000,000 | 8.642 | 115.71M | 8.862 | 112.84M | 10.168 | 1.18× | 1.15× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.787 ms**; native kernel **0.778 ms**; TA-Lib 1.153 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.353 | 0.280 | 3.57M | 990.371 | 3535.64× | 105.89× |
| 100,000 | 10 | 2.869 | 1.401 | 7.14M | 1075.704 | 767.83× | 19.82× |
| 100,000 | 1,000 | 31.217 | 26.116 | 38.29M | 1008.572 | 38.62× | 1.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.71M | 105.52M | 1.00× | 2.28M | 2.23M | 1.00× | 82.72M |
| 2 | 199.31M | 180.46M | 1.71× | 2.39M | 2.42M | 1.08× | 80.63M |
| 4 | 345.82M | 373.84M | 3.54× | 2.20M | 2.42M | 1.08× | 79.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
