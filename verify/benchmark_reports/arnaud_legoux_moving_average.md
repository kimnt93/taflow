# ArnaudLegouxMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.35M | 0.012 | 85.16M | nan | — | — |
| 10,000 | 0.103 | 97.35M | 0.104 | 96.50M | nan | — | — |
| 100,000 | 1.018 | 98.21M | 0.978 | 102.29M | nan | — | — |
| 1,000,000 | 10.268 | 97.39M | 9.908 | 100.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.020 ms**; native kernel **0.985 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.167 | 6.00M | nan | — | — |
| 100,000 | 10 | 1.048 | 0.627 | 15.96M | nan | — | — |
| 100,000 | 1,000 | 12.694 | 11.728 | 85.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 76.39M | 86.84M | 1.00× | 3.31M | 2.88M | 1.00× | — |
| 2 | 149.87M | 175.50M | 2.02× | 3.31M | 3.53M | 1.23× | — |
| 4 | 282.41M | 311.34M | 3.59× | 3.31M | 3.46M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
