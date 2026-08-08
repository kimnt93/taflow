# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 248.70M | 0.003 | 304.55M | nan | — | — |
| 10,000 | 0.022 | 445.93M | 0.024 | 424.88M | nan | — | — |
| 100,000 | 0.206 | 486.09M | 0.215 | 464.11M | nan | — | — |
| 1,000,000 | 2.379 | 420.40M | 2.365 | 422.88M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.207 ms**; native kernel **0.234 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.586 | 0.312 | 3.21M | nan | — | — |
| 100,000 | 10 | 1.271 | 0.792 | 12.62M | nan | — | — |
| 100,000 | 1,000 | 4.658 | 4.801 | 208.31M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 307.68M | 373.22M | 1.00× | 2.16M | 2.46M | 1.00× | — |
| 2 | 525.28M | 732.58M | 1.96× | 2.25M | 2.37M | 0.96× | — |
| 4 | 611.07M | 1.21G | 3.23× | 2.35M | 2.46M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
