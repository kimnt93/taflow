# Donchian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.66M | 0.006 | 180.66M | nan | — | — |
| 10,000 | 0.056 | 179.83M | 0.047 | 214.45M | nan | — | — |
| 100,000 | 0.538 | 185.71M | 0.432 | 231.54M | nan | — | — |
| 1,000,000 | 16.334 | 61.22M | 6.506 | 153.70M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.548 ms**; native kernel **0.445 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.379 | 0.313 | 3.19M | nan | — | — |
| 100,000 | 10 | 2.410 | 1.845 | 5.42M | nan | — | — |
| 100,000 | 1,000 | 98.304 | 90.053 | 11.10M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78.78M | 140.63M | 1.00× | 1.77M | 1.36M | 1.00× | — |
| 2 | 130.39M | 305.05M | 2.17× | 1.87M | 1.96M | 1.44× | — |
| 4 | 145.80M | 294.52M | 2.09× | 1.31M | 1.30M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
