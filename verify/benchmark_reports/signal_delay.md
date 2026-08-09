# SignalDelay benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.76M | 0.006 | 156.69M | nan | — | — |
| 10,000 | 0.046 | 219.00M | 0.042 | 238.86M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.258 | 0.180 | 5.56M | nan | — | — |
| 1,500 | 10 | 1.234 | 1.106 | 9.04M | nan | — | — |
| 1,500 | 100 | 2.151 | 1.574 | 63.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.30M | 17.41M | 1.00× | 1.45M | 1.54M | 1.00× | — |
| 2 | 19.45M | 23.18M | 1.33× | 1.27M | 1.40M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
