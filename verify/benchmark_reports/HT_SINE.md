# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.133 | 7.54M | 0.134 | 7.46M | 0.483 | 3.64× | 3.60× |
| 10,000 | 1.433 | 6.98M | 1.357 | 7.37M | 4.641 | 3.24× | 3.42× |
| 100,000 | 13.359 | 7.49M | 13.722 | 7.29M | 47.444 | 3.55× | 3.46× |
| 1,000,000 | 140.806 | 7.10M | 140.712 | 7.11M | 472.380 | 3.35× | 3.36× |

## Warm-up

Construct + canonical extend over 100,000 bars: **14.284 ms**; native kernel **13.977 ms**; TA-Lib 49.489 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.419 | 0.341 | 2.94M | 46354.733 | 136060.07× | 112.71× |
| 100,000 | 10 | 2.460 | 2.093 | 4.78M | 46957.561 | 22431.65× | 20.69× |
| 100,000 | 1,000 | 143.334 | 150.964 | 6.62M | 47434.580 | 314.21× | 3.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.61M | 6.72M | 1.00× | 1.69M | 1.87M | 1.00× | 2.03M |
| 2 | 12.14M | 12.63M | 1.88× | 1.67M | 1.69M | 0.90× | 2.08M |
| 4 | 24.96M | 23.62M | 3.52× | 1.90M | 1.64M | 0.88× | 2.01M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
