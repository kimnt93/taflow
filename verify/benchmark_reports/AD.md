# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 226.61M | 0.003 | 376.98M | 0.028 | 6.28× | 10.45× |
| 10,000 | 0.018 | 540.65M | 0.014 | 700.52M | 0.041 | 2.21× | 2.86× |
| 100,000 | 0.147 | 682.33M | 0.122 | 822.65M | 0.147 | 1.00× | 1.21× |
| 1,000,000 | 2.276 | 439.43M | 1.815 | 551.01M | 1.887 | 0.83× | 1.04× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.149 ms**; native kernel **0.121 ms**; TA-Lib 0.147 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.396 | 0.256 | 3.90M | 153.706 | 599.86× | 102.83× |
| 100,000 | 10 | 2.407 | 1.150 | 8.69M | 144.687 | 125.77× | 22.43× |
| 100,000 | 1,000 | 5.655 | 3.304 | 302.62M | 148.378 | 44.90× | 8.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 304.71M | 345.02M | 1.00× | 2.00M | 2.86M | 1.00× | 296.95M |
| 2 | 575.95M | 801.45M | 2.32× | 2.06M | 2.79M | 0.98× | 357.20M |
| 4 | 669.70M | 1.06G | 3.09× | 2.04M | 2.57M | 0.90× | 391.11M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
