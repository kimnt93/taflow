# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.86M | 0.005 | 215.97M | 0.040 | 6.13× | 8.66× |
| 10,000 | 0.098 | 101.85M | 0.092 | 108.19M | 0.172 | 1.76× | 1.86× |
| 100,000 | 1.155 | 86.58M | 1.121 | 89.21M | 1.426 | 1.23× | 1.27× |
| 1,000,000 | 11.896 | 84.06M | 11.769 | 84.97M | 14.186 | 1.19× | 1.21× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.143 ms**; native kernel **1.168 ms**; TA-Lib 1.435 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.351 | 0.292 | 3.42M | 1400.449 | 4790.60× | 95.07× |
| 100,000 | 10 | 2.671 | 1.577 | 6.34M | 1423.767 | 902.90× | 18.24× |
| 100,000 | 1,000 | 35.448 | 28.758 | 34.77M | 1504.011 | 52.30× | 1.42× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 77.58M | 81.01M | 1.00× | 2.17M | 2.31M | 1.00× | 63.21M |
| 2 | 155.18M | 163.69M | 2.02× | 2.34M | 2.56M | 1.11× | 60.99M |
| 4 | 264.84M | 293.17M | 3.62× | 2.29M | 2.47M | 1.07× | 62.43M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
