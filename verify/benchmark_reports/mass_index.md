# MassIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.02M | 0.007 | 143.11M | nan | — | — |
| 10,000 | 0.061 | 163.76M | 0.060 | 167.87M | nan | — | — |
| 100,000 | 0.600 | 166.72M | 0.607 | 164.80M | nan | — | — |
| 1,000,000 | 6.408 | 156.04M | 5.746 | 174.03M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.593 ms**; native kernel **0.572 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.269 | 0.210 | 4.77M | nan | — | — |
| 100,000 | 10 | 1.590 | 0.804 | 12.44M | nan | — | — |
| 100,000 | 1,000 | 11.905 | 7.382 | 135.46M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 121.41M | 147.03M | 1.00× | 2.51M | 2.61M | 1.00× | — |
| 2 | 247.87M | 283.29M | 1.93× | 2.91M | 2.94M | 1.13× | — |
| 4 | 140.08M | 149.06M | 1.01× | 2.78M | 2.87M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
