# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.49M | 0.005 | 188.38M | 0.039 | 5.36× | 7.30× |
| 10,000 | 0.122 | 82.24M | 0.116 | 86.01M | 0.170 | 1.40× | 1.46× |
| 100,000 | 1.279 | 78.17M | 1.248 | 80.11M | 1.430 | 1.12× | 1.15× |
| 1,000,000 | 13.275 | 75.33M | 12.775 | 78.28M | 14.236 | 1.07× | 1.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.252 ms**; native kernel **1.239 ms**; TA-Lib 1.436 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.330 | 0.264 | 3.79M | 1426.378 | 5406.55× | 104.98× |
| 100,000 | 10 | 2.953 | 1.338 | 7.47M | 1442.157 | 1077.50× | 20.36× |
| 100,000 | 1,000 | 30.277 | 24.195 | 41.33M | 1445.496 | 59.74× | 1.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.77M | 69.89M | 1.00× | 1.93M | 2.40M | 1.00× | 63.17M |
| 2 | 143.34M | 143.11M | 2.05× | 2.32M | 2.61M | 1.09× | 61.51M |
| 4 | 233.56M | 256.95M | 3.68× | 2.32M | 2.42M | 1.01× | 62.03M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
