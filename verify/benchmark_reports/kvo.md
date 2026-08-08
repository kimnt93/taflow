# KlingerVolumeOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.19M | 0.012 | 85.20M | nan | — | — |
| 10,000 | 0.104 | 96.57M | 0.099 | 101.29M | nan | — | — |
| 100,000 | 1.018 | 98.24M | 0.887 | 112.79M | nan | — | — |
| 1,000,000 | 11.035 | 90.62M | 10.579 | 94.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.009 ms**; native kernel **1.002 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.430 | 0.435 | 2.30M | nan | — | — |
| 100,000 | 10 | 2.318 | 1.279 | 7.82M | nan | — | — |
| 100,000 | 1,000 | 15.245 | 11.169 | 89.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.17M | 88.85M | 1.00× | 1.82M | 2.32M | 1.00× | — |
| 2 | 81.43M | 94.87M | 1.07× | 1.82M | 2.37M | 1.02× | — |
| 4 | 80.43M | 91.11M | 1.03× | 1.77M | 2.16M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
