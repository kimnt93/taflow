# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.19M | 0.024 | 42.04M | nan | — | — |
| 10,000 | 0.218 | 45.95M | 0.204 | 49.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.034 ms**; native kernel **0.033 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.265 | 0.173 | 5.79M | nan | — | — |
| 1,500 | 10 | 0.854 | 0.618 | 16.17M | nan | — | — |
| 1,500 | 100 | 3.279 | 2.955 | 33.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.16M | 10.38M | 1.00× | 1.39M | 955.62K | 1.00× | — |
| 2 | 12.25M | 15.05M | 1.45× | 1.06M | 1.51M | 1.58× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
