# OutsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.41M | 0.006 | 159.99M | nan | — | — |
| 10,000 | 0.037 | 271.45M | 0.033 | 303.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.444 | 0.282 | 3.55M | nan | — | — |
| 1,500 | 10 | 1.691 | 0.823 | 12.16M | nan | — | — |
| 1,500 | 100 | 2.906 | 1.631 | 61.30M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.26M | 15.23M | 1.00× | 1.12M | 1.50M | 1.00× | — |
| 2 | 14.78M | 21.79M | 1.43× | 1.36M | 1.66M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
