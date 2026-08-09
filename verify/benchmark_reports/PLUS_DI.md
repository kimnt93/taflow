# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.75M | 0.007 | 150.55M | 0.037 | 4.58× | 5.58× |
| 10,000 | 0.060 | 166.43M | 0.056 | 178.53M | 0.094 | 1.57× | 1.69× |
| 100,000 | 0.572 | 174.90M | 0.546 | 183.08M | 0.698 | 1.22× | 1.28× |
| 1,000,000 | 6.126 | 163.23M | 5.774 | 173.18M | 6.542 | 1.07× | 1.13× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.578 ms**; native kernel **0.544 ms**; TA-Lib 0.650 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.270 | 0.212 | 4.72M | 656.966 | 3102.17× | 141.86× |
| 100,000 | 10 | 1.871 | 0.933 | 10.71M | 656.092 | 702.88× | 32.34× |
| 100,000 | 1,000 | 9.124 | 7.318 | 136.66M | 655.935 | 89.64× | 5.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 114.61M | 135.76M | 1.00× | 2.67M | 2.99M | 1.00× | 121.74M |
| 2 | 235.54M | 284.84M | 2.10× | 2.66M | 2.73M | 0.91× | 122.55M |
| 4 | 404.29M | 446.47M | 3.29× | 2.54M | 2.39M | 0.80× | 121.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
