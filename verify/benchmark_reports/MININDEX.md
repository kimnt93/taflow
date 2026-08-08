# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.01M | 0.010 | 103.16M | 0.036 | 2.85× | 3.72× |
| 10,000 | 0.180 | 55.48M | 0.150 | 66.70M | 0.097 | 0.54× | 0.65× |
| 100,000 | 1.814 | 55.12M | 1.515 | 66.03M | 0.751 | 0.41× | 0.50× |
| 1,000,000 | 21.145 | 47.29M | 15.797 | 63.30M | 6.968 | 0.33× | 0.44× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.778 ms**; native kernel **1.512 ms**; TA-Lib 0.683 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.159 | 6.28M | 719.564 | 4516.60× | 176.44× |
| 100,000 | 10 | 1.016 | 0.664 | 15.06M | 702.307 | 1057.68× | 43.78× |
| 100,000 | 1,000 | 20.887 | 15.620 | 64.02M | 724.106 | 46.36× | 2.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 58.12M | 62.50M | 1.00× | 2.73M | 3.42M | 1.00× | 121.21M |
| 2 | 113.11M | 120.01M | 1.92× | 2.62M | 3.08M | 0.90× | 114.74M |
| 4 | 175.17M | 204.72M | 3.28× | 2.61M | 2.69M | 0.79× | 116.58M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
