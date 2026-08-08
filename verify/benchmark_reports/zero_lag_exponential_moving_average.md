# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.62M | 0.006 | 177.86M | nan | — | — |
| 10,000 | 0.050 | 201.47M | 0.042 | 239.41M | nan | — | — |
| 100,000 | 0.448 | 223.09M | 0.393 | 254.22M | nan | — | — |
| 1,000,000 | 4.820 | 207.47M | 4.149 | 241.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.443 ms**; native kernel **0.398 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.216 | 0.154 | 6.51M | nan | — | — |
| 100,000 | 10 | 0.977 | 0.569 | 17.58M | nan | — | — |
| 100,000 | 1,000 | 6.204 | 10.032 | 99.69M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 189.60M | 220.60M | 1.00× | 3.42M | 3.53M | 1.00× | — |
| 2 | 334.41M | 410.58M | 1.86× | 3.48M | 3.97M | 1.13× | — |
| 4 | 479.06M | 740.77M | 3.36× | 3.80M | 3.84M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
