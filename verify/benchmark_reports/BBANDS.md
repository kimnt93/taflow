# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.75M | 0.007 | 133.64M | 0.053 | 6.05× | 7.05× |
| 10,000 | 0.072 | 139.56M | 0.065 | 154.60M | 0.106 | 1.48× | 1.64× |
| 100,000 | 0.701 | 142.62M | 0.632 | 158.21M | 0.558 | 0.80× | 0.88× |
| 1,000,000 | 19.814 | 50.47M | 10.331 | 96.80M | 9.965 | 0.50× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.681 ms**; native kernel **0.583 ms**; TA-Lib 0.548 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.362 | 0.271 | 3.69M | 556.778 | 2055.64× | 167.43× |
| 100,000 | 10 | 1.545 | 1.249 | 8.01M | 539.008 | 431.50× | 38.45× |
| 100,000 | 1,000 | 129.666 | 100.705 | 9.93M | 543.584 | 5.40× | 0.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 105.96M | 134.37M | 1.00× | 1.50M | 1.85M | 1.00× | 129.01M |
| 2 | 170.41M | 252.33M | 1.88× | 1.64M | 1.48M | 0.80× | 130.85M |
| 4 | 232.18M | 497.86M | 3.71× | 1.31M | 1.49M | 0.81× | 138.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
