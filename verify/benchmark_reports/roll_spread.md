# RollSpread benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.17M | 0.045 | 22.16M | nan | — | — |
| 10,000 | 0.425 | 23.54M | 0.437 | 22.86M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.068 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.213 | 4.69M | nan | — | — |
| 1,500 | 10 | 1.351 | 0.930 | 10.76M | nan | — | — |
| 1,500 | 100 | 5.981 | 5.297 | 18.88M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.02M | 8.85M | 1.00× | 1.09M | 1.39M | 1.00× | — |
| 2 | 9.67M | 14.37M | 1.62× | 962.07K | 1.30M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
