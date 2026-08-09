# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.097 | 10.30M | 0.094 | 10.61M | 0.432 | 4.45× | 4.58× |
| 10,000 | 0.970 | 10.31M | 0.983 | 10.17M | 4.231 | 4.36× | 4.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.149 ms**; native kernel **0.143 ms**; TA-Lib 0.630 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.356 | 0.271 | 3.69M | 641.519 | 2368.07× | 122.98× |
| 1,500 | 10 | 2.038 | 1.585 | 6.31M | 641.537 | 404.87× | 24.40× |
| 1,500 | 100 | 12.955 | 11.625 | 8.60M | 683.156 | 58.77× | 7.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.04M | 4.56M | 1.00× | 1.00M | 1.32M | 1.00× | 1.75M |
| 2 | 10.16M | 8.49M | 1.86× | 1.33M | 1.12M | 0.85× | 1.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
