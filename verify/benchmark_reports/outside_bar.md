# OutsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.71M | 0.004 | 245.97M | nan | — | — |
| 10,000 | 0.034 | 294.55M | 0.030 | 338.01M | nan | — | — |
| 100,000 | 0.305 | 327.49M | 0.281 | 356.15M | nan | — | — |
| 1,000,000 | 3.735 | 267.72M | 3.322 | 301.06M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.312 ms**; native kernel **0.290 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.284 | 0.192 | 5.20M | nan | — | — |
| 100,000 | 10 | 1.530 | 0.789 | 12.68M | nan | — | — |
| 100,000 | 1,000 | 5.497 | 4.399 | 227.33M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 223.73M | 275.97M | 1.00× | 3.11M | 3.45M | 1.00× | — |
| 2 | 362.10M | 537.99M | 1.95× | 3.20M | 3.40M | 0.99× | — |
| 4 | 504.75M | 593.11M | 2.15× | 3.26M | 3.26M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
