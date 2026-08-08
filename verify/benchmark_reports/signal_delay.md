# SignalDelay benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.81M | 0.005 | 217.66M | nan | — | — |
| 10,000 | 0.038 | 264.13M | 0.035 | 287.06M | nan | — | — |
| 100,000 | 0.365 | 274.22M | 0.338 | 296.04M | nan | — | — |
| 1,000,000 | 3.811 | 262.40M | 3.505 | 285.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.356 ms**; native kernel **0.330 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.219 | 0.164 | 6.10M | nan | — | — |
| 100,000 | 10 | 0.998 | 0.589 | 16.97M | nan | — | — |
| 100,000 | 1,000 | 5.691 | 4.830 | 207.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 207.47M | 231.66M | 1.00× | 3.25M | 3.66M | 1.00× | — |
| 2 | 347.50M | 480.66M | 2.07× | 3.36M | 3.97M | 1.09× | — |
| 4 | 505.44M | 588.09M | 2.54× | 3.74M | 3.96M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
