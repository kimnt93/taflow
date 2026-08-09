# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.37M | 0.008 | 129.69M | nan | — | — |
| 10,000 | 0.064 | 155.83M | 0.060 | 166.01M | nan | — | — |
| 100,000 | 0.601 | 166.51M | 0.578 | 172.87M | nan | — | — |
| 1,000,000 | 6.686 | 149.57M | 6.094 | 164.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.604 ms**; native kernel **0.580 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.337 | 0.255 | 3.92M | nan | — | — |
| 100,000 | 10 | 2.480 | 1.174 | 8.52M | nan | — | — |
| 100,000 | 1,000 | 9.595 | 10.115 | 98.86M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.77M | 138.42M | 1.00× | 2.19M | 2.37M | 1.00× | — |
| 2 | 117.14M | 131.01M | 0.95× | 2.63M | 2.67M | 1.13× | — |
| 4 | 144.83M | 136.50M | 0.99× | 2.41M | 2.47M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
