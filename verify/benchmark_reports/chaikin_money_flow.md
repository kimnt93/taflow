# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.29M | 0.016 | 60.62M | nan | — | — |
| 10,000 | 0.078 | 127.54M | 0.075 | 133.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.015 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.413 | 0.285 | 3.51M | nan | — | — |
| 1,500 | 10 | 2.585 | 1.273 | 7.85M | nan | — | — |
| 1,500 | 100 | 4.275 | 2.808 | 35.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.12M | 11.48M | 1.00× | 1.12M | 908.95K | 1.00× | — |
| 2 | 17.85M | 21.06M | 1.83× | 1.10M | 1.44M | 1.58× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
