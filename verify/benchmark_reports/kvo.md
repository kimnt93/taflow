# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.48M | 0.015 | 68.18M | nan | — | — |
| 10,000 | 0.109 | 91.95M | 0.101 | 98.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.466 | 0.302 | 3.31M | nan | — | — |
| 1,500 | 10 | 1.769 | 1.174 | 8.52M | nan | — | — |
| 1,500 | 100 | 3.679 | 2.898 | 34.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.88M | 12.54M | 1.00× | 740.94K | 1.24M | 1.00× | — |
| 2 | 14.86M | 16.13M | 1.29× | 965.45K | 1.30M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
