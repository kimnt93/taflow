# CumulativeCount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 231.12M | 0.003 | 324.08M | nan | — | — |
| 10,000 | 0.025 | 399.84M | 0.021 | 467.21M | nan | — | — |
| 100,000 | 0.226 | 443.08M | 0.200 | 499.98M | nan | — | — |
| 1,000,000 | 2.538 | 393.96M | 2.200 | 454.64M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.230 ms**; native kernel **0.204 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.236 | 0.167 | 5.99M | nan | — | — |
| 100,000 | 10 | 0.955 | 0.530 | 18.88M | nan | — | — |
| 100,000 | 1,000 | 4.094 | 3.248 | 307.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 271.02M | 353.28M | 1.00× | 2.96M | 4.17M | 1.00× | — |
| 2 | 547.28M | 715.93M | 2.03× | 3.33M | 3.91M | 0.94× | — |
| 4 | 610.18M | 1.13G | 3.20× | 3.47M | 3.84M | 0.92× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
