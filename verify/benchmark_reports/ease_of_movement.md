# EaseOfMovement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.84M | 0.007 | 153.13M | nan | — | — |
| 10,000 | 0.038 | 265.94M | 0.032 | 313.02M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.319 | 0.256 | 3.91M | nan | — | — |
| 1,500 | 10 | 1.999 | 0.994 | 10.06M | nan | — | — |
| 1,500 | 100 | 3.019 | 1.988 | 50.29M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.68M | 16.62M | 1.00× | 1.36M | 1.09M | 1.00× | — |
| 2 | 16.53M | 21.33M | 1.28× | 1.44M | 1.31M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
