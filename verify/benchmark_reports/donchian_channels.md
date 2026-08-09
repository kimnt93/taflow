# DonchianChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.57M | 0.009 | 110.03M | nan | — | — |
| 10,000 | 0.107 | 93.20M | 0.087 | 115.16M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.399 | 0.285 | 3.51M | nan | — | — |
| 1,500 | 10 | 1.938 | 1.089 | 9.19M | nan | — | — |
| 1,500 | 100 | 5.616 | 4.570 | 21.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.60M | 15.92M | 1.00× | 1.06M | 1.15M | 1.00× | — |
| 2 | 16.57M | 16.33M | 1.03× | 1.27M | 1.39M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
