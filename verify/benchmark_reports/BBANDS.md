# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.22M | 0.006 | 158.34M | 0.061 | 8.56× | 9.74× |
| 10,000 | 0.046 | 216.53M | 0.039 | 258.74M | 0.099 | 2.14× | 2.56× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**; TA-Lib 0.061 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.318 | 0.226 | 4.43M | 55.857 | 247.42× | 208.56× |
| 1,500 | 10 | 0.856 | 0.719 | 13.91M | 55.928 | 77.79× | 67.05× |
| 1,500 | 100 | 3.582 | 2.659 | 37.61M | 56.894 | 21.40× | 17.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.79M | 13.75M | 1.00× | 1.01M | 936.23K | 1.00× | 8.04M |
| 2 | 20.21M | 16.83M | 1.22× | 1.28M | 1.38M | 1.47× | 7.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
