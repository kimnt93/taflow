# LowestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 182.71M | 0.004 | 232.35M | nan | — | — |
| 10,000 | 0.036 | 279.86M | 0.034 | 297.02M | nan | — | — |
| 100,000 | 0.357 | 280.07M | 0.326 | 306.54M | nan | — | — |
| 1,000,000 | 3.887 | 257.29M | 3.399 | 294.24M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.351 ms**; native kernel **0.324 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.300 | 3.33M | nan | — | — |
| 100,000 | 10 | 1.244 | 0.760 | 13.16M | nan | — | — |
| 100,000 | 1,000 | 6.130 | 4.814 | 207.73M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 174.73M | 209.03M | 1.00× | 2.16M | 2.13M | 1.00× | — |
| 2 | 325.88M | 433.83M | 2.08× | 2.22M | 2.57M | 1.21× | — |
| 4 | 496.89M | 669.14M | 3.20× | 2.30M | 2.35M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
