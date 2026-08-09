# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.096 | 10.44M | 0.095 | 10.54M | nan | — | — |
| 10,000 | 0.911 | 10.98M | 0.906 | 11.04M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.138 ms**; native kernel **0.135 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.340 | 0.254 | 3.94M | nan | — | — |
| 1,500 | 10 | 1.691 | 1.334 | 7.50M | nan | — | — |
| 1,500 | 100 | 9.879 | 9.560 | 10.46M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 4.41M | 5.05M | 1.00× | 910.65K | 756.75K | 1.00× | — |
| 2 | 6.75M | 6.38M | 1.26× | 1.26M | 1.33M | 1.75× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
