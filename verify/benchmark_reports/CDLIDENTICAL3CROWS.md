# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.89M | 0.005 | 198.81M | 0.035 | 4.78× | 7.00× |
| 10,000 | 0.074 | 134.89M | 0.068 | 146.55M | 0.120 | 1.62× | 1.75× |
| 100,000 | 0.794 | 125.90M | 0.793 | 126.15M | 0.944 | 1.19× | 1.19× |
| 1,000,000 | 8.575 | 116.62M | 8.111 | 123.30M | 9.384 | 1.09× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.796 ms**; native kernel **0.761 ms**; TA-Lib 0.922 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.356 | 0.286 | 3.50M | 925.058 | 3239.25× | 99.74× |
| 100,000 | 10 | 2.748 | 1.560 | 6.41M | 971.079 | 622.45× | 17.26× |
| 100,000 | 1,000 | 31.478 | 28.865 | 34.64M | 958.067 | 33.19× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.31M | 106.58M | 1.00× | 2.15M | 2.22M | 1.00× | 94.35M |
| 2 | 200.11M | 214.24M | 2.01× | 2.29M | 2.65M | 1.19× | 90.14M |
| 4 | 356.87M | 411.22M | 3.86× | 2.21M | 2.34M | 1.06× | 95.09M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
