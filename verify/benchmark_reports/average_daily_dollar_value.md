# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.18M | 0.008 | 119.28M | nan | — | — |
| 10,000 | 0.051 | 196.97M | 0.048 | 206.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.271 | 0.200 | 5.00M | nan | — | — |
| 1,500 | 10 | 1.512 | 1.854 | 5.40M | nan | — | — |
| 1,500 | 100 | 2.598 | 1.922 | 52.02M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.41M | 12.38M | 1.00× | 1.35M | 931.95K | 1.00× | — |
| 2 | 16.90M | 21.11M | 1.71× | 1.13M | 1.39M | 1.50× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
