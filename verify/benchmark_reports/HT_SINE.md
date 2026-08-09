# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.131 | 7.62M | 0.129 | 7.75M | 0.467 | 3.56× | 3.62× |
| 10,000 | 1.328 | 7.53M | 1.315 | 7.60M | 4.427 | 3.33× | 3.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.202 ms**; native kernel **0.191 ms**; TA-Lib 0.748 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.735 | 0.633 | 1.58M | 691.911 | 1092.27× | 59.68× |
| 1,500 | 10 | 2.429 | 1.939 | 5.16M | 702.360 | 362.14× | 21.59× |
| 1,500 | 100 | 16.625 | 14.886 | 6.72M | 707.922 | 47.56× | 6.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 3.56M | 3.33M | 1.00× | 785.35K | 1.20M | 1.00× | 1.57M |
| 2 | 8.02M | 7.11M | 2.13× | 1.03M | 1.01M | 0.84× | 1.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
