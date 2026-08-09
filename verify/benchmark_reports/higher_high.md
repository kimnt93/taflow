# HigherHigh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.76M | 0.006 | 170.54M | nan | — | — |
| 10,000 | 0.033 | 307.24M | 0.029 | 343.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.410 | 0.200 | 5.00M | nan | — | — |
| 1,500 | 10 | 1.617 | 0.778 | 12.86M | nan | — | — |
| 1,500 | 100 | 2.546 | 1.668 | 59.94M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.42M | 14.30M | 1.00× | 1.07M | 1.12M | 1.00× | — |
| 2 | 18.39M | 21.24M | 1.49× | 1.26M | 1.49M | 1.34× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
