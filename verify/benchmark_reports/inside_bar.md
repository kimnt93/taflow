# InsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.80M | 0.006 | 171.87M | nan | — | — |
| 10,000 | 0.032 | 307.92M | 0.029 | 341.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.203 | 4.93M | nan | — | — |
| 1,500 | 10 | 2.060 | 0.782 | 12.79M | nan | — | — |
| 1,500 | 100 | 2.627 | 1.741 | 57.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.05M | 14.53M | 1.00× | 1.25M | 1.47M | 1.00× | — |
| 2 | 18.04M | 21.28M | 1.46× | 887.65K | 1.56M | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
