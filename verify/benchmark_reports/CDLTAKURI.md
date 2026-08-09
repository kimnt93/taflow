# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.32M | 0.004 | 242.96M | 0.036 | 5.46× | 8.65× |
| 10,000 | 0.056 | 179.63M | 0.053 | 189.95M | 0.105 | 1.88× | 1.99× |
| 100,000 | 0.620 | 161.28M | 0.610 | 163.91M | 0.794 | 1.28× | 1.30× |
| 1,000,000 | 6.463 | 154.72M | 6.338 | 157.78M | 8.042 | 1.24× | 1.27× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.610 ms**; native kernel **0.602 ms**; TA-Lib 0.798 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.334 | 0.284 | 3.52M | 787.350 | 2767.60× | 98.38× |
| 100,000 | 10 | 2.489 | 1.292 | 7.74M | 792.671 | 613.67× | 21.78× |
| 100,000 | 1,000 | 27.522 | 28.729 | 34.81M | 791.603 | 27.55× | 1.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.74M | 132.77M | 1.00× | 2.14M | 2.39M | 1.00× | 105.42M |
| 2 | 256.70M | 274.23M | 2.07× | 2.30M | 2.66M | 1.11× | 103.85M |
| 4 | 464.72M | 497.29M | 3.75× | 2.54M | 2.79M | 1.17× | 106.81M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
