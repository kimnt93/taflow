# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.96M | 0.057 | 17.62M | nan | — | — |
| 10,000 | 0.548 | 18.23M | 0.553 | 18.07M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.087 ms**; native kernel **0.094 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.399 | 0.225 | 4.44M | nan | — | — |
| 1,500 | 10 | 1.503 | 1.117 | 8.96M | nan | — | — |
| 1,500 | 100 | 7.344 | 6.957 | 14.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.96M | 9.65M | 1.00× | 1.34M | 1.52M | 1.00× | — |
| 2 | 10.86M | 12.47M | 1.29× | 1.15M | 1.30M | 0.86× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
