# PivotPoints benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.76M | 0.012 | 85.94M | nan | — | — |
| 10,000 | 0.105 | 94.83M | 0.098 | 101.79M | nan | — | — |
| 100,000 | 1.101 | 90.85M | 0.927 | 107.86M | nan | — | — |
| 1,000,000 | 35.037 | 28.54M | 22.678 | 44.10M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.123 ms**; native kernel **0.931 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.418 | 0.389 | 2.57M | nan | — | — |
| 100,000 | 10 | 1.710 | 1.033 | 9.68M | nan | — | — |
| 100,000 | 1,000 | 10.994 | 9.886 | 101.15M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 65.10M | 82.62M | 1.00× | 1.60M | 1.85M | 1.00× | — |
| 2 | 62.83M | 83.42M | 1.01× | 1.71M | 1.81M | 0.98× | — |
| 4 | 66.65M | 80.84M | 0.98× | 1.57M | 1.55M | 0.84× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
