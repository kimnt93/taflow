# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.23M | 0.013 | 75.41M | nan | — | — |
| 10,000 | 0.127 | 79.02M | 0.118 | 84.45M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.020 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.563 | 0.477 | 2.10M | nan | — | — |
| 1,500 | 10 | 2.196 | 1.371 | 7.30M | nan | — | — |
| 1,500 | 100 | 4.391 | 3.413 | 29.30M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
