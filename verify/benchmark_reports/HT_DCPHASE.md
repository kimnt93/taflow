# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.435 | 2.30M | 0.417 | 2.40M | 0.482 | 1.11× | 1.16× |
| 10,000 | 4.463 | 2.24M | 4.320 | 2.31M | 4.315 | 0.97× | 1.00× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.623 ms**; native kernel **0.610 ms**; TA-Lib 0.681 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.728 | 0.882 | 1.13M | 669.889 | 759.58× | 56.99× |
| 1,500 | 10 | 5.715 | 5.251 | 1.90M | 709.156 | 135.05× | 7.82× |
| 1,500 | 100 | 50.472 | 49.228 | 2.03M | 706.054 | 14.34× | 1.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
