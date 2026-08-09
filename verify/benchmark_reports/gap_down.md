# GapDown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.00M | 0.004 | 280.98M | nan | — | — |
| 10,000 | 0.029 | 342.47M | 0.026 | 390.42M | nan | — | — |
| 100,000 | 0.281 | 355.49M | 0.261 | 382.51M | nan | — | — |
| 1,000,000 | 3.148 | 317.65M | 2.818 | 354.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.288 ms**; native kernel **0.257 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.177 | 5.65M | nan | — | — |
| 100,000 | 10 | 1.356 | 0.699 | 14.31M | nan | — | — |
| 100,000 | 1,000 | 4.941 | 6.734 | 148.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 216.49M | 301.47M | 1.00× | 3.31M | 3.51M | 1.00× | — |
| 2 | 464.24M | 473.67M | 1.57× | 3.22M | 3.55M | 1.01× | — |
| 4 | 570.20M | 973.07M | 3.23× | 3.41M | 3.50M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
