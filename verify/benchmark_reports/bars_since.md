# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.11M | 0.004 | 223.41M | nan | — | — |
| 10,000 | 0.031 | 324.38M | 0.028 | 355.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.337 | 0.329 | 3.04M | nan | — | — |
| 1,500 | 10 | 0.740 | 0.540 | 18.53M | nan | — | — |
| 1,500 | 100 | 1.573 | 1.349 | 74.11M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.93M | 7.21M | 1.00× | 1.34M | 1.37M | 1.00× | — |
| 2 | 21.00M | 24.70M | 3.42× | 1.15M | 1.52M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
