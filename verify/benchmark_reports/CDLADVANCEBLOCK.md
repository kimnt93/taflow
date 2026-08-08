# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.29M | 0.042 | 23.72M | 0.049 | 0.79× | 1.15× |
| 10,000 | 0.414 | 24.17M | 0.400 | 24.98M | 0.236 | 0.57× | 0.59× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.064 ms**; native kernel **0.059 ms**; TA-Lib 0.058 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.423 | 0.345 | 2.90M | 58.322 | 169.13× | 83.24× |
| 1,500 | 10 | 2.944 | 1.495 | 6.69M | 60.073 | 40.19× | 19.92× |
| 1,500 | 100 | 9.091 | 6.350 | 15.75M | 104.902 | 16.52× | 4.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
