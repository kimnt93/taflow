# VolumeWeightedMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.79M | 0.016 | 62.42M | nan | — | — |
| 10,000 | 0.129 | 77.37M | 0.129 | 77.74M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.024 ms**; native kernel **0.022 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.280 | 0.207 | 4.82M | nan | — | — |
| 1,500 | 10 | 2.105 | 0.892 | 11.21M | nan | — | — |
| 1,500 | 100 | 3.528 | 2.620 | 38.16M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.71M | 14.65M | 1.00× | 951.84K | 1.24M | 1.00× | — |
| 2 | 15.41M | 15.99M | 1.09× | 1.40M | 1.50M | 1.21× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
