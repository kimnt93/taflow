# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.66M | 0.012 | 80.72M | nan | — | — |
| 10,000 | 0.108 | 92.21M | 0.111 | 90.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.018 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.176 | 5.70M | nan | — | — |
| 1,500 | 10 | 1.181 | 0.661 | 15.13M | nan | — | — |
| 1,500 | 100 | 3.459 | 2.584 | 38.69M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.57M | 9.30M | 1.00× | 1.08M | 865.85K | 1.00× | — |
| 2 | 14.37M | 22.28M | 2.40× | 1.13M | 1.23M | 1.42× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
