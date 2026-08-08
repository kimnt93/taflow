# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.74M | 0.007 | 142.94M | 0.034 | 4.15× | 4.91× |
| 10,000 | 0.053 | 187.22M | 0.052 | 194.13M | 0.100 | 1.86× | 1.93× |
| 100,000 | 0.488 | 204.99M | 0.472 | 211.90M | 0.612 | 1.25× | 1.30× |
| 1,000,000 | 5.595 | 178.74M | 5.288 | 189.10M | 6.292 | 1.12× | 1.19× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.490 ms**; native kernel **0.478 ms**; TA-Lib 0.608 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.433 | 0.304 | 3.29M | 608.870 | 2003.88× | 92.83× |
| 100,000 | 10 | 2.689 | 1.429 | 7.00M | 608.576 | 426.01× | 19.45× |
| 100,000 | 1,000 | 25.028 | 22.531 | 44.38M | 604.062 | 26.81× | 1.34× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 168.04M | 146.08M | 1.00× | 2.17M | 2.30M | 1.00× | 135.92M |
| 2 | 318.00M | 336.55M | 2.30× | 2.22M | 2.40M | 1.04× | 133.35M |
| 4 | 524.43M | 618.58M | 4.23× | 2.33M | 2.69M | 1.17× | 133.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
