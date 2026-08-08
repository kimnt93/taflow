# PivotPoints benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.22M | 0.011 | 90.87M | nan | — | — |
| 10,000 | 0.107 | 93.37M | 0.089 | 112.65M | nan | — | — |
| 100,000 | 1.066 | 93.81M | 0.833 | 119.98M | nan | — | — |
| 1,000,000 | 30.342 | 32.96M | 9.790 | 102.15M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.114 ms**; native kernel **0.828 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.445 | 0.423 | 2.36M | nan | — | — |
| 100,000 | 10 | 1.745 | 1.157 | 8.65M | nan | — | — |
| 100,000 | 1,000 | 14.814 | 10.817 | 92.45M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.06M | 82.45M | 1.00× | 1.51M | 1.71M | 1.00× | — |
| 2 | 62.60M | 82.85M | 1.00× | 1.44M | 1.72M | 1.00× | — |
| 4 | 66.64M | 83.86M | 1.02× | 1.70M | 1.68M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
