# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.84M | 0.010 | 100.66M | 0.038 | 3.29× | 3.82× |
| 10,000 | 0.072 | 139.33M | 0.066 | 152.48M | 0.088 | 1.22× | 1.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.014 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.407 | 0.236 | 4.23M | 40.759 | 172.50× | 130.35× |
| 1,500 | 10 | 2.137 | 1.048 | 9.54M | 41.553 | 39.65× | 30.75× |
| 1,500 | 100 | 4.461 | 2.636 | 37.94M | 51.530 | 19.55× | 13.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.66M | 15.02M | 1.00× | 1.07M | 1.37M | 1.00× | 9.58M |
| 2 | 17.16M | 21.76M | 1.45× | 1.22M | 1.54M | 1.12× | 9.30M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
