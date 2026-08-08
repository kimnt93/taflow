# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.41M | 0.010 | 102.93M | 0.033 | 2.77× | 3.41× |
| 10,000 | 0.124 | 80.35M | 0.123 | 81.32M | 0.083 | 0.67× | 0.68× |
| 100,000 | 1.187 | 84.26M | 1.168 | 85.63M | 0.595 | 0.50× | 0.51× |
| 1,000,000 | 12.210 | 81.90M | 11.692 | 85.53M | 5.931 | 0.49× | 0.51× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.217 ms**; native kernel **1.185 ms**; TA-Lib 0.589 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.370 | 0.302 | 3.31M | 609.775 | 2017.83× | 92.71× |
| 100,000 | 10 | 2.836 | 1.580 | 6.33M | 592.344 | 374.92× | 17.71× |
| 100,000 | 1,000 | 49.321 | 27.355 | 36.56M | 585.354 | 21.40× | 1.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 76.72M | 74.44M | 1.00× | 2.35M | 2.30M | 1.00× | 138.01M |
| 2 | 146.76M | 149.40M | 2.01× | 2.42M | 2.46M | 1.07× | 139.56M |
| 4 | 190.55M | 205.43M | 2.76× | 2.13M | 2.35M | 1.02× | 141.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
