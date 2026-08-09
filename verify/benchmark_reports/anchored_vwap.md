# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.82M | 0.017 | 59.45M | nan | — | — |
| 10,000 | 0.127 | 78.85M | 0.117 | 85.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.023 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.520 | 0.511 | 1.96M | nan | — | — |
| 1,500 | 10 | 2.139 | 1.368 | 7.31M | nan | — | — |
| 1,500 | 100 | 3.973 | 3.172 | 31.52M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.38M | 13.92M | 1.00× | 759.31K | 836.90K | 1.00× | — |
| 2 | 13.74M | 13.96M | 1.00× | 940.43K | 850.83K | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
