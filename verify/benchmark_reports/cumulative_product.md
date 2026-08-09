# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 241.47M | 0.003 | 314.40M | nan | — | — |
| 10,000 | 0.026 | 386.32M | 0.022 | 447.69M | nan | — | — |
| 100,000 | 0.246 | 406.95M | 0.222 | 451.11M | nan | — | — |
| 1,000,000 | 2.703 | 369.97M | 2.277 | 439.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.252 ms**; native kernel **0.225 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.191 | 0.152 | 6.58M | nan | — | — |
| 100,000 | 10 | 0.846 | 0.491 | 20.35M | nan | — | — |
| 100,000 | 1,000 | 4.059 | 3.385 | 295.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 284.56M | 293.22M | 1.00× | 4.16M | 4.10M | 1.00× | — |
| 2 | 477.06M | 580.24M | 1.98× | 3.78M | 3.72M | 0.91× | — |
| 4 | 568.56M | 946.18M | 3.23× | 3.77M | 4.09M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
