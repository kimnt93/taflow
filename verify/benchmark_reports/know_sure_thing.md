# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.44M | 0.021 | 48.43M | nan | — | — |
| 10,000 | 0.176 | 56.70M | 0.207 | 48.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.030 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.333 | 0.284 | 3.52M | nan | — | — |
| 1,500 | 10 | 1.313 | 0.839 | 11.92M | nan | — | — |
| 1,500 | 100 | 4.340 | 3.787 | 26.41M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.88M | 13.82M | 1.00× | 1.17M | 1.28M | 1.00× | — |
| 2 | 15.75M | 16.99M | 1.23× | 1.24M | 1.27M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
