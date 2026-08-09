# ExponentiallyWeightedStandardDeviation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.24M | 0.007 | 139.37M | nan | — | — |
| 10,000 | 0.056 | 179.80M | 0.051 | 197.77M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.227 | 0.227 | 4.41M | nan | — | — |
| 1,500 | 10 | 0.963 | 0.549 | 18.21M | nan | — | — |
| 1,500 | 100 | 2.096 | 2.131 | 46.93M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.00M | 14.56M | 1.00× | 1.12M | 1.42M | 1.00× | — |
| 2 | 18.86M | 21.41M | 1.47× | 1.66M | 1.74M | 1.23× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
