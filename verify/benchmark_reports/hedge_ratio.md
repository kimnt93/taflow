# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.84M | 0.040 | 25.06M | nan | — | — |
| 10,000 | 0.387 | 25.84M | 0.390 | 25.67M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.061 ms**; native kernel **0.062 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.328 | 0.256 | 3.91M | nan | — | — |
| 1,500 | 10 | 1.805 | 1.086 | 9.20M | nan | — | — |
| 1,500 | 100 | 5.870 | 5.025 | 19.90M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.46M | 7.54M | 1.00× | 1.16M | 989.08K | 1.00× | — |
| 2 | 11.08M | 15.43M | 2.05× | 1.08M | 1.39M | 1.40× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
