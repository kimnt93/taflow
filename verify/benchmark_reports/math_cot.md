# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.80M | 0.020 | 51.06M | nan | — | — |
| 10,000 | 0.205 | 48.81M | 0.207 | 48.33M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.030 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.322 | 0.188 | 5.33M | nan | — | — |
| 1,500 | 10 | 1.335 | 0.784 | 12.75M | nan | — | — |
| 1,500 | 100 | 4.424 | 3.599 | 27.79M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.34M | 11.39M | 1.00× | 742.19K | 1.50M | 1.00× | — |
| 2 | 16.85M | 15.88M | 1.39× | 1.43M | 1.63M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
