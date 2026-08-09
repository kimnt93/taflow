# YangZhang benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.64M | 0.057 | 17.45M | nan | — | — |
| 10,000 | 0.471 | 21.23M | 0.467 | 21.42M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.076 ms**; native kernel **0.072 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.395 | 0.486 | 2.06M | nan | — | — |
| 1,500 | 10 | 2.881 | 2.193 | 4.56M | nan | — | — |
| 1,500 | 100 | 7.569 | 6.042 | 16.55M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.82M | 6.74M | 1.00× | 625.47K | 956.75K | 1.00× | — |
| 2 | 12.81M | 12.41M | 1.84× | 1.22M | 1.19M | 1.24× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
