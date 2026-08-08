# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.83M | 0.011 | 92.80M | nan | — | — |
| 10,000 | 0.100 | 100.34M | 0.096 | 103.87M | nan | — | — |
| 100,000 | 1.115 | 89.73M | 0.948 | 105.50M | nan | — | — |
| 1,000,000 | 30.411 | 32.88M | 19.587 | 51.06M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.138 ms**; native kernel **0.935 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.489 | 0.374 | 2.68M | nan | — | — |
| 100,000 | 10 | 1.816 | 1.165 | 8.58M | nan | — | — |
| 100,000 | 1,000 | 11.691 | 12.288 | 81.38M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 71.78M | 92.74M | 1.00× | 1.76M | 2.09M | 1.00× | — |
| 2 | 66.48M | 78.37M | 0.85× | 1.72M | 2.22M | 1.06× | — |
| 4 | 67.77M | 83.01M | 0.90× | 1.64M | 2.03M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
