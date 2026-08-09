# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.95M | 0.022 | 44.72M | 0.057 | 2.37× | 2.53× |
| 10,000 | 0.189 | 52.95M | 0.189 | 52.89M | 0.243 | 1.29× | 1.28× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.031 ms**; TA-Lib 0.063 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.247 | 4.04M | 60.434 | 244.19× | 121.34× |
| 1,500 | 10 | 2.695 | 1.188 | 8.42M | 63.174 | 53.19× | 25.45× |
| 1,500 | 100 | 6.037 | 4.191 | 23.86M | 62.329 | 14.87× | 7.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.03M | 9.29M | 1.00× | 944.49K | 812.70K | 1.00× | 7.96M |
| 2 | 15.08M | 17.18M | 1.85× | 1.13M | 1.44M | 1.77× | 8.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
