# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.86M | 0.004 | 241.61M | 0.030 | 5.16× | 7.29× |
| 10,000 | 0.030 | 331.14M | 0.027 | 374.81M | 0.050 | 1.67× | 1.89× |
| 100,000 | 0.259 | 386.31M | 0.255 | 391.83M | 0.243 | 0.94× | 0.95× |
| 1,000,000 | 3.160 | 316.41M | 2.992 | 334.26M | 2.556 | 0.81× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.266 ms**; native kernel **0.247 ms**; TA-Lib 0.242 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.336 | 0.267 | 3.75M | 241.081 | 903.61× | 102.43× |
| 100,000 | 10 | 2.563 | 1.275 | 7.84M | 240.827 | 188.88× | 21.63× |
| 100,000 | 1,000 | 18.640 | 16.393 | 61.00M | 242.017 | 14.76× | 1.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 205.28M | 284.92M | 1.00× | 2.48M | 2.48M | 1.00× | 270.21M |
| 2 | 499.04M | 465.84M | 1.64× | 2.50M | 2.91M | 1.18× | 292.35M |
| 4 | 762.67M | 809.03M | 2.84× | 2.50M | 2.73M | 1.10× | 272.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
