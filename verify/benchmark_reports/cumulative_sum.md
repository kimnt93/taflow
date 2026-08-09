# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 247.87M | 0.003 | 321.04M | nan | — | — |
| 10,000 | 0.025 | 398.04M | 0.022 | 453.03M | nan | — | — |
| 100,000 | 0.248 | 403.66M | 0.229 | 436.69M | nan | — | — |
| 1,000,000 | 2.687 | 372.16M | 2.316 | 431.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.251 ms**; native kernel **0.226 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.191 | 0.139 | 7.21M | nan | — | — |
| 100,000 | 10 | 0.864 | 0.473 | 21.12M | nan | — | — |
| 100,000 | 1,000 | 5.011 | 3.438 | 290.89M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 196.92M | 298.24M | 1.00× | 3.24M | 3.75M | 1.00× | — |
| 2 | 443.71M | 575.32M | 1.93× | 3.33M | 3.79M | 1.01× | — |
| 4 | 630.03M | 758.47M | 2.54× | 3.86M | 4.17M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
