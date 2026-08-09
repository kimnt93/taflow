# ForceIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.92M | 0.006 | 160.71M | nan | — | — |
| 10,000 | 0.037 | 269.64M | 0.033 | 304.48M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.281 | 0.206 | 4.86M | nan | — | — |
| 1,500 | 10 | 1.534 | 0.779 | 12.84M | nan | — | — |
| 1,500 | 100 | 2.576 | 1.768 | 56.56M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.85M | 14.61M | 1.00× | 1.09M | 1.22M | 1.00× | — |
| 2 | 17.84M | 19.19M | 1.31× | 1.47M | 1.60M | 1.31× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
