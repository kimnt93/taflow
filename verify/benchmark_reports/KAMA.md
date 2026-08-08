# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.42M | 0.006 | 167.20M | 0.033 | 4.76× | 5.59× |
| 10,000 | 0.051 | 194.21M | 0.055 | 180.23M | 0.061 | 1.18× | 1.10× |
| 100,000 | 0.512 | 195.19M | 0.473 | 211.48M | 0.323 | 0.63× | 0.68× |
| 1,000,000 | 5.266 | 189.91M | 4.707 | 212.44M | 3.071 | 0.58× | 0.65× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.497 ms**; native kernel **0.471 ms**; TA-Lib 0.316 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.284 | 0.165 | 6.06M | 325.064 | 1968.54× | 183.27× |
| 100,000 | 10 | 0.911 | 0.562 | 17.79M | 321.843 | 572.45× | 54.88× |
| 100,000 | 1,000 | 7.169 | 6.328 | 158.04M | 339.814 | 53.70× | 5.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 155.20M | 176.02M | 1.00× | 3.37M | 3.32M | 1.00× | 234.27M |
| 2 | 273.26M | 351.80M | 2.00× | 3.43M | 3.53M | 1.06× | 222.90M |
| 4 | 404.59M | 609.96M | 3.47× | 3.05M | 2.90M | 0.87× | 235.37M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
