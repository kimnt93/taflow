# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 255.01M | 0.003 | 346.00M | 0.031 | 7.88× | 10.69× |
| 10,000 | 0.023 | 436.49M | 0.020 | 496.82M | 0.041 | 1.79× | 2.04× |
| 100,000 | 0.211 | 474.36M | 0.186 | 538.99M | 0.126 | 0.60× | 0.68× |
| 1,000,000 | 2.389 | 418.52M | 1.988 | 503.02M | 1.172 | 0.49× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.214 ms**; native kernel **0.193 ms**; TA-Lib 0.125 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.157 | 6.39M | 125.992 | 804.91× | 175.94× |
| 100,000 | 10 | 0.916 | 0.498 | 20.08M | 124.385 | 249.81× | 58.20× |
| 100,000 | 1,000 | 4.233 | 3.112 | 321.35M | 126.190 | 40.55× | 9.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 297.62M | 433.31M | 1.00× | 3.24M | 4.25M | 1.00× | 419.52M |
| 2 | 608.82M | 730.81M | 1.69× | 3.17M | 3.88M | 0.91× | 437.26M |
| 4 | 746.59M | 1.19G | 2.74× | 3.12M | 3.58M | 0.84× | 466.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
