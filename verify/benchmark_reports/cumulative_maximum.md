# CumulativeMaximum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.07M | 0.005 | 219.74M | nan | — | — |
| 10,000 | 0.041 | 246.05M | 0.038 | 262.13M | nan | — | — |
| 100,000 | 0.382 | 262.07M | 0.361 | 276.87M | nan | — | — |
| 1,000,000 | 4.361 | 229.29M | 3.704 | 269.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.381 ms**; native kernel **0.363 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.193 | 0.150 | 6.67M | nan | — | — |
| 100,000 | 10 | 0.833 | 0.488 | 20.51M | nan | — | — |
| 100,000 | 1,000 | 5.538 | 7.093 | 140.98M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 192.08M | 187.85M | 1.00× | 2.98M | 3.09M | 1.00× | — |
| 2 | 336.65M | 398.44M | 2.12× | 3.52M | 3.65M | 1.18× | — |
| 4 | 501.62M | 726.73M | 3.87× | 3.96M | 3.91M | 1.27× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
