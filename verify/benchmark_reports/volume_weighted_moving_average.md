# VolumeWeightedMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.38M | 0.011 | 91.04M | nan | — | — |
| 10,000 | 0.098 | 102.00M | 0.098 | 102.26M | nan | — | — |
| 100,000 | 0.909 | 110.00M | 0.905 | 110.52M | nan | — | — |
| 1,000,000 | 9.651 | 103.62M | 9.389 | 106.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.910 ms**; native kernel **0.903 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.191 | 5.24M | nan | — | — |
| 100,000 | 10 | 1.647 | 0.824 | 12.13M | nan | — | — |
| 100,000 | 1,000 | 13.230 | 11.033 | 90.63M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 87.04M | 97.29M | 1.00× | 3.23M | 2.75M | 1.00× | — |
| 2 | 163.65M | 183.52M | 1.89× | 2.93M | 2.92M | 1.06× | — |
| 4 | 170.98M | 183.21M | 1.88× | 3.03M | 2.90M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
