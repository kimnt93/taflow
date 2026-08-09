# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.76M | 0.006 | 166.55M | nan | — | — |
| 10,000 | 0.034 | 296.78M | 0.030 | 333.92M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.202 | 4.95M | nan | — | — |
| 1,500 | 10 | 1.554 | 0.748 | 13.37M | nan | — | — |
| 1,500 | 100 | 2.632 | 1.619 | 61.77M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.82M | 15.73M | 1.00× | 1.26M | 1.44M | 1.00× | — |
| 2 | 17.96M | 20.46M | 1.30× | 1.29M | 1.58M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
