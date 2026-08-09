# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.40M | 0.017 | 59.59M | nan | — | — |
| 10,000 | 0.117 | 85.48M | 0.115 | 87.07M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.025 ms**; native kernel **0.022 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.410 | 0.309 | 3.23M | nan | — | — |
| 1,500 | 10 | 2.469 | 1.725 | 5.80M | nan | — | — |
| 1,500 | 100 | 4.341 | 2.916 | 34.29M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.48M | 12.76M | 1.00× | 1.08M | 526.39K | 1.00× | — |
| 2 | 15.28M | 13.42M | 1.05× | 1.10M | 1.11M | 2.12× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
