# ExponentiallyWeightedCovariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.26M | 0.010 | 104.94M | nan | — | — |
| 10,000 | 0.062 | 160.76M | 0.054 | 183.53M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.011 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.282 | 0.203 | 4.93M | nan | — | — |
| 1,500 | 10 | 1.528 | 0.805 | 12.42M | nan | — | — |
| 1,500 | 100 | 2.838 | 2.497 | 40.05M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.57M | 10.43M | 1.00× | 1.20M | 756.40K | 1.00× | — |
| 2 | 16.89M | 15.51M | 1.49× | 1.42M | 1.46M | 1.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
