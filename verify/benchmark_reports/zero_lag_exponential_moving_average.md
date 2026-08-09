# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.23M | 0.006 | 177.43M | nan | — | — |
| 10,000 | 0.048 | 208.22M | 0.045 | 224.41M | nan | — | — |
| 100,000 | 0.570 | 175.53M | 0.423 | 236.49M | nan | — | — |
| 1,000,000 | 5.224 | 191.41M | 4.476 | 223.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.502 ms**; native kernel **0.417 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.206 | 0.162 | 6.17M | nan | — | — |
| 100,000 | 10 | 0.872 | 0.522 | 19.16M | nan | — | — |
| 100,000 | 1,000 | 6.295 | 5.376 | 186.01M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.00M | 184.23M | 1.00× | 3.02M | 3.21M | 1.00× | — |
| 2 | 310.13M | 347.42M | 1.89× | 3.67M | 3.89M | 1.21× | — |
| 4 | 393.93M | 627.83M | 3.41× | 3.79M | 3.81M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
