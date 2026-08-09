# ArnaudLegouxMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.10M | 0.017 | 57.93M | nan | — | — |
| 10,000 | 0.145 | 68.80M | 0.141 | 70.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.025 ms**; native kernel **0.024 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.234 | 0.171 | 5.85M | nan | — | — |
| 1,500 | 10 | 1.095 | 0.824 | 12.13M | nan | — | — |
| 1,500 | 100 | 3.110 | 2.499 | 40.01M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.88M | 12.53M | 1.00× | 1.03M | 1.24M | 1.00× | — |
| 2 | 14.76M | 17.50M | 1.40× | 1.27M | 1.64M | 1.32× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
