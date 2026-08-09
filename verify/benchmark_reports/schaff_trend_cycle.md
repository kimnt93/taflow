# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.07M | 0.053 | 18.75M | nan | — | — |
| 10,000 | 0.613 | 16.32M | 0.602 | 16.62M | nan | — | — |
| 100,000 | 6.458 | 15.48M | 6.145 | 16.27M | nan | — | — |
| 1,000,000 | 77.406 | 12.92M | 61.362 | 16.30M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.196 ms**; native kernel **6.235 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.420 | 0.370 | 2.70M | nan | — | — |
| 100,000 | 10 | 2.537 | 1.881 | 5.32M | nan | — | — |
| 100,000 | 1,000 | 133.576 | 139.721 | 7.16M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 14.52M | 15.75M | 1.00× | 1.43M | 1.47M | 1.00× | — |
| 2 | 27.67M | 29.98M | 1.90× | 1.55M | 1.53M | 1.04× | — |
| 4 | 51.15M | 57.72M | 3.67× | 1.57M | 1.38M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
