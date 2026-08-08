# DetrendedPriceOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.02M | 0.006 | 174.90M | nan | — | — |
| 10,000 | 0.050 | 201.06M | 0.048 | 210.02M | nan | — | — |
| 100,000 | 0.477 | 209.52M | 0.456 | 219.36M | nan | — | — |
| 1,000,000 | 5.046 | 198.17M | 4.580 | 218.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.485 ms**; native kernel **0.456 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.173 | 5.76M | nan | — | — |
| 100,000 | 10 | 0.987 | 0.592 | 16.89M | nan | — | — |
| 100,000 | 1,000 | 6.931 | 6.099 | 163.96M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 152.42M | 196.80M | 1.00× | 3.30M | 3.20M | 1.00× | — |
| 2 | 287.80M | 352.45M | 1.79× | 3.71M | 3.43M | 1.07× | — |
| 4 | 433.03M | 306.30M | 1.56× | 3.62M | 3.68M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
