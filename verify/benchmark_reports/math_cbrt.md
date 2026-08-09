# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.39M | 0.020 | 49.05M | nan | — | — |
| 10,000 | 0.182 | 54.86M | 0.185 | 54.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.030 ms**; native kernel **0.029 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.313 | 0.180 | 5.54M | nan | — | — |
| 1,500 | 10 | 1.239 | 0.739 | 13.54M | nan | — | — |
| 1,500 | 100 | 4.030 | 4.145 | 24.13M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.26M | 13.31M | 1.00× | 1.08M | 1.25M | 1.00× | — |
| 2 | 12.52M | 17.49M | 1.31× | 1.24M | 1.76M | 1.41× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
