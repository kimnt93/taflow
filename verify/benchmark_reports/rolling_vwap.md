# RollingVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.41M | 0.038 | 26.21M | nan | — | — |
| 10,000 | 0.399 | 25.06M | 0.428 | 23.36M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.059 ms**; native kernel **0.060 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.419 | 0.313 | 3.20M | nan | — | — |
| 1,500 | 10 | 2.775 | 1.422 | 7.03M | nan | — | — |
| 1,500 | 100 | 7.454 | 9.126 | 10.96M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
