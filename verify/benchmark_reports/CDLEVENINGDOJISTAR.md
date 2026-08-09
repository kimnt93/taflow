# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.37M | 0.009 | 105.98M | 0.042 | 2.74× | 4.45× |
| 10,000 | 0.100 | 99.76M | 0.139 | 71.81M | 0.209 | 2.08× | 1.50× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.021 ms**; TA-Lib 0.078 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.850 | 0.654 | 1.53M | 81.678 | 124.88× | 98.31× |
| 1,500 | 10 | 6.720 | 3.087 | 3.24M | 81.657 | 26.46× | 16.69× |
| 1,500 | 100 | 11.057 | 6.545 | 15.28M | 86.339 | 13.19× | 10.04× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.40M | 8.33M | 1.00× | 563.10K | 621.01K | 1.00× | 5.17M |
| 2 | 9.75M | 11.26M | 1.35× | 706.04K | 697.30K | 1.12× | 5.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
