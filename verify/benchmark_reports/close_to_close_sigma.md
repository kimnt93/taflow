# CloseToCloseSigma benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.86M | 0.019 | 53.76M | nan | — | — |
| 10,000 | 0.180 | 55.55M | 0.177 | 56.63M | nan | — | — |
| 100,000 | 1.750 | 57.15M | 1.736 | 57.61M | nan | — | — |
| 1,000,000 | 17.806 | 56.16M | 17.295 | 57.82M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.758 ms**; native kernel **1.737 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.171 | 5.86M | nan | — | — |
| 100,000 | 10 | 1.045 | 0.667 | 14.98M | nan | — | — |
| 100,000 | 1,000 | 19.403 | 18.720 | 53.42M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.06M | 53.49M | 1.00× | 2.81M | 2.96M | 1.00× | — |
| 2 | 92.77M | 101.29M | 1.89× | 3.23M | 3.03M | 1.03× | — |
| 4 | 83.73M | 87.21M | 1.63× | 3.21M | 3.23M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
