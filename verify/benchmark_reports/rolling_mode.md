# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.210 | 4.77M | 0.224 | 4.47M | nan | — | — |
| 10,000 | 2.066 | 4.84M | 2.104 | 4.75M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.321 ms**; native kernel **0.331 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.433 | 0.367 | 2.72M | nan | — | — |
| 1,500 | 10 | 3.104 | 2.611 | 3.83M | nan | — | — |
| 1,500 | 100 | 21.992 | 31.439 | 3.18M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 3.28M | 3.20M | 1.00× | 810.27K | 1.21M | 1.00× | — |
| 2 | 5.35M | 6.01M | 1.88× | 1.06M | 1.17M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
