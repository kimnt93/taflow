# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.31M | 0.008 | 126.76M | 0.047 | 4.87× | 5.97× |
| 10,000 | 0.086 | 115.86M | 0.079 | 126.51M | 0.153 | 1.78× | 1.94× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.377 | 0.282 | 3.55M | 40.832 | 144.79× | 131.45× |
| 1,500 | 10 | 3.196 | 1.452 | 6.89M | 40.600 | 27.96× | 25.28× |
| 1,500 | 100 | 6.139 | 3.551 | 28.16M | 57.056 | 16.07× | 10.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.08M | 13.39M | 1.00× | 891.04K | 1.21M | 1.00× | 7.01M |
| 2 | 11.94M | 15.92M | 1.19× | 944.51K | 1.07M | 0.88× | 7.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
