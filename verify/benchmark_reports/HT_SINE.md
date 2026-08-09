# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.131 | 7.66M | 0.127 | 7.88M | 0.459 | 3.51× | 3.61× |
| 10,000 | 1.422 | 7.03M | 1.342 | 7.45M | 4.465 | 3.14× | 3.33× |
| 100,000 | 13.448 | 7.44M | 13.414 | 7.45M | 45.293 | 3.37× | 3.38× |
| 1,000,000 | 135.123 | 7.40M | 133.031 | 7.52M | 445.534 | 3.30× | 3.35× |

## Warm-up

Construct + canonical extend over 100,000 bars: **13.254 ms**; native kernel **13.270 ms**; TA-Lib 44.166 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.392 | 0.319 | 3.14M | 44973.374 | 141119.18× | 113.50× |
| 100,000 | 10 | 2.397 | 1.887 | 5.30M | 43898.103 | 23261.11× | 20.92× |
| 100,000 | 1,000 | 144.645 | 134.707 | 7.42M | 45189.804 | 335.47× | 3.75× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.02M | 7.31M | 1.00× | 1.88M | 2.03M | 1.00× | 1.99M |
| 2 | 13.41M | 13.93M | 1.91× | 1.81M | 1.83M | 0.90× | 2.22M |
| 4 | 25.82M | 24.14M | 3.30× | 1.87M | 1.95M | 0.96× | 2.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
