# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.514 | 1.95M | 0.520 | 1.92M | 0.513 | 1.00× | 0.99× |
| 10,000 | 5.182 | 1.93M | 5.129 | 1.95M | 4.954 | 0.96× | 0.97× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.733 ms**; native kernel **0.760 ms**; TA-Lib 0.715 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.804 | 0.715 | 1.40M | 720.173 | 1006.80× | 50.90× |
| 1,500 | 10 | 6.278 | 11.396 | 877.52K | 727.440 | 63.83× | 3.67× |
| 1,500 | 100 | 55.006 | 53.267 | 1.88M | 821.911 | 15.43× | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
