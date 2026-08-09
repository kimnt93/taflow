# ZeroLagExponentialMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.32M | 0.007 | 137.13M | nan | — | — |
| 10,000 | 0.056 | 178.38M | 0.053 | 189.30M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.240 | 0.171 | 5.84M | nan | — | — |
| 1,500 | 10 | 1.004 | 0.600 | 16.67M | nan | — | — |
| 1,500 | 100 | 2.165 | 1.651 | 60.58M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.75M | 17.44M | 1.00× | 1.44M | 1.54M | 1.00× | — |
| 2 | 20.33M | 20.92M | 1.20× | 1.63M | 1.70M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
