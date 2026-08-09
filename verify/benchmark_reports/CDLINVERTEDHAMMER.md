# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.89M | 0.005 | 213.81M | 0.040 | 6.14× | 8.58× |
| 10,000 | 0.119 | 84.16M | 0.112 | 89.66M | 0.167 | 1.40× | 1.50× |
| 100,000 | 1.203 | 83.15M | 1.181 | 84.68M | 1.404 | 1.17× | 1.19× |
| 1,000,000 | 12.212 | 81.88M | 12.718 | 78.63M | 13.835 | 1.13× | 1.09× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.191 ms**; native kernel **1.191 ms**; TA-Lib 1.406 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.328 | 0.273 | 3.66M | 1399.413 | 5117.51× | 101.66× |
| 100,000 | 10 | 2.653 | 1.374 | 7.28M | 1396.802 | 1016.87× | 20.46× |
| 100,000 | 1,000 | 32.787 | 28.001 | 35.71M | 1437.183 | 51.33× | 1.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 68.99M | 72.90M | 1.00× | 2.05M | 2.21M | 1.00× | 63.17M |
| 2 | 138.12M | 147.68M | 2.03× | 2.37M | 2.64M | 1.19× | 64.62M |
| 4 | 247.29M | 268.60M | 3.68× | 2.27M | 2.45M | 1.11× | 63.06M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
