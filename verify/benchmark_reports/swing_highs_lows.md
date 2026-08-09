# SwingHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.62M | 0.037 | 26.96M | nan | — | — |
| 10,000 | 0.399 | 25.05M | 0.391 | 25.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.057 ms**; native kernel **0.058 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.413 | 0.379 | 2.64M | nan | — | — |
| 1,500 | 10 | 2.017 | 1.124 | 8.89M | nan | — | — |
| 1,500 | 100 | 6.025 | 5.042 | 19.83M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.64M | 9.86M | 1.00× | 788.54K | 689.08K | 1.00× | — |
| 2 | 11.65M | 15.43M | 1.56× | 1.12M | 1.31M | 1.90× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
