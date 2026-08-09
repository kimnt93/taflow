# CumulativeCount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.33M | 0.004 | 224.17M | nan | — | — |
| 10,000 | 0.023 | 428.53M | 0.021 | 480.54M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.244 | 0.161 | 6.21M | nan | — | — |
| 1,500 | 10 | 0.929 | 0.533 | 18.75M | nan | — | — |
| 1,500 | 100 | 1.760 | 1.262 | 79.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 14.49M | 14.24M | 1.00× | 1.09M | 1.12M | 1.00× | — |
| 2 | 17.86M | 21.80M | 1.53× | 1.58M | 1.54M | 1.37× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
