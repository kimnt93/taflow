# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 219.14M | 0.003 | 289.01M | nan | — | — |
| 10,000 | 0.015 | 649.21M | 0.013 | 754.21M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.291 | 0.161 | 6.22M | nan | — | — |
| 1,500 | 10 | 1.060 | 0.566 | 17.67M | nan | — | — |
| 1,500 | 100 | 2.505 | 1.687 | 59.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.21M | 15.04M | 1.00× | 1.05M | 1.46M | 1.00× | — |
| 2 | 20.46M | 22.46M | 1.49× | 1.35M | 1.70M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
