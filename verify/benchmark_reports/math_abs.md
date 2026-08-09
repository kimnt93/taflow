# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.15M | 0.004 | 284.24M | nan | — | — |
| 10,000 | 0.017 | 597.46M | 0.013 | 766.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.450 | 0.210 | 4.77M | nan | — | — |
| 1,500 | 10 | 1.094 | 0.572 | 17.47M | nan | — | — |
| 1,500 | 100 | 2.410 | 1.706 | 58.61M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.62M | 8.50M | 1.00× | 1.26M | 1.52M | 1.00× | — |
| 2 | 19.72M | 22.44M | 2.64× | 1.09M | 1.70M | 1.12× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
