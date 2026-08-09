# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.44M | 0.005 | 192.43M | nan | — | — |
| 10,000 | 0.028 | 354.12M | 0.024 | 417.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.331 | 0.295 | 3.39M | nan | — | — |
| 1,500 | 10 | 1.367 | 0.759 | 13.17M | nan | — | — |
| 1,500 | 100 | 2.407 | 1.538 | 65.01M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.83M | 17.09M | 1.00× | 1.23M | 1.20M | 1.00× | — |
| 2 | 11.91M | 18.38M | 1.08× | 1.39M | 1.31M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
