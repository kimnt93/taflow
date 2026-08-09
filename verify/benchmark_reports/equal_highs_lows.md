# EqualHighsLows benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.60M | 0.046 | 21.68M | nan | — | — |
| 10,000 | 0.505 | 19.81M | 0.501 | 19.96M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.077 ms**; native kernel **0.072 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.426 | 0.366 | 2.73M | nan | — | — |
| 1,500 | 10 | 3.206 | 1.509 | 6.63M | nan | — | — |
| 1,500 | 100 | 7.613 | 6.236 | 16.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.14M | 6.11M | 1.00× | 782.09K | 1.12M | 1.00× | — |
| 2 | 11.40M | 12.65M | 2.07× | 1.27M | 1.24M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
