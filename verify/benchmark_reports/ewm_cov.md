# ExponentiallyWeightedCovariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.55M | 0.006 | 160.58M | nan | — | — |
| 10,000 | 0.052 | 192.73M | 0.048 | 206.87M | nan | — | — |
| 100,000 | 0.527 | 189.71M | 0.470 | 212.81M | nan | — | — |
| 1,000,000 | 5.614 | 178.12M | 5.077 | 196.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.492 ms**; native kernel **0.468 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.209 | 4.79M | nan | — | — |
| 100,000 | 10 | 1.491 | 0.810 | 12.35M | nan | — | — |
| 100,000 | 1,000 | 8.396 | 6.480 | 154.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.17M | 177.01M | 1.00× | 2.94M | 3.15M | 1.00× | — |
| 2 | 268.51M | 235.05M | 1.33× | 3.25M | 3.32M | 1.06× | — |
| 4 | 294.14M | 548.03M | 3.10× | 3.00M | 3.20M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
