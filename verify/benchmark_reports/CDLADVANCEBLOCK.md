# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.65M | 0.006 | 157.05M | 0.048 | 5.92× | 7.58× |
| 10,000 | 0.086 | 116.78M | 0.087 | 114.82M | 0.227 | 2.65× | 2.60× |
| 100,000 | 0.914 | 109.37M | 0.871 | 114.83M | 1.936 | 2.12× | 2.22× |
| 1,000,000 | 9.866 | 101.36M | 9.294 | 107.59M | 19.602 | 1.99× | 2.11× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.876 ms**; native kernel **0.852 ms**; TA-Lib 1.954 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.361 | 0.278 | 3.59M | 1947.777 | 6997.18× | 98.20× |
| 100,000 | 10 | 2.751 | 1.370 | 7.30M | 2055.967 | 1500.18× | 19.96× |
| 100,000 | 1,000 | 34.209 | 33.623 | 29.74M | 1923.378 | 57.20× | 1.39× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.68M | 99.17M | 1.00× | 2.01M | 2.34M | 1.00× | 45.22M |
| 2 | 184.57M | 198.31M | 2.00× | 2.39M | 2.38M | 1.01× | 47.04M |
| 4 | 330.27M | 345.74M | 3.49× | 2.19M | 2.33M | 0.99× | 46.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
