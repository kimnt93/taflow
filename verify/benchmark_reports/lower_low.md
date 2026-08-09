# LowerLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 205.25M | 0.003 | 296.44M | nan | — | — |
| 10,000 | 0.029 | 344.33M | 0.026 | 388.09M | nan | — | — |
| 100,000 | 0.267 | 374.29M | 0.246 | 406.61M | nan | — | — |
| 1,000,000 | 3.079 | 324.79M | 2.762 | 362.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.269 ms**; native kernel **0.246 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.246 | 0.188 | 5.32M | nan | — | — |
| 100,000 | 10 | 1.635 | 0.687 | 14.56M | nan | — | — |
| 100,000 | 1,000 | 5.157 | 3.949 | 253.26M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 227.80M | 325.77M | 1.00× | 3.03M | 3.38M | 1.00× | — |
| 2 | 387.50M | 515.44M | 1.58× | 3.50M | 3.62M | 1.07× | — |
| 4 | 602.23M | 944.03M | 2.90× | 3.44M | 3.56M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
