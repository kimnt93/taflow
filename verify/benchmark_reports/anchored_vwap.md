# AnchoredVolumeWeightedAveragePrice benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.99M | 0.016 | 61.47M | nan | — | — |
| 10,000 | 0.145 | 68.77M | 0.117 | 85.84M | nan | — | — |
| 100,000 | 1.211 | 82.55M | 1.088 | 91.94M | nan | — | — |
| 1,000,000 | 25.902 | 38.61M | 12.398 | 80.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.213 ms**; native kernel **1.080 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.504 | 0.443 | 2.26M | nan | — | — |
| 100,000 | 10 | 2.452 | 1.298 | 7.70M | nan | — | — |
| 100,000 | 1,000 | 14.055 | 12.895 | 77.55M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 66.38M | 76.88M | 1.00× | 1.42M | 1.50M | 1.00× | — |
| 2 | 66.41M | 78.13M | 1.02× | 1.55M | 1.66M | 1.10× | — |
| 4 | 63.71M | 74.44M | 0.97× | 1.48M | 1.58M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
