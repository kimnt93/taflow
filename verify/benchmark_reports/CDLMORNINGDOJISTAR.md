# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.83M | 0.009 | 111.90M | 0.039 | 3.70× | 4.32× |
| 10,000 | 0.088 | 114.04M | 0.084 | 118.72M | 0.119 | 1.36× | 1.42× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.451 | 0.268 | 3.73M | 42.563 | 158.66× | 128.72× |
| 1,500 | 10 | 2.588 | 1.279 | 7.82M | 44.657 | 34.91× | 26.65× |
| 1,500 | 100 | 19.305 | 3.351 | 29.84M | 45.384 | 13.54× | 10.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.69M | 8.30M | 1.00× | 1.21M | 956.85K | 1.00× | 6.15M |
| 2 | 18.86M | 18.26M | 2.20× | 1.25M | 1.38M | 1.44× | 9.51M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
