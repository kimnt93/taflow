# FairValueGap benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.11M | 0.012 | 83.20M | nan | — | — |
| 10,000 | 0.112 | 88.95M | 0.104 | 96.09M | nan | — | — |
| 100,000 | 1.142 | 87.57M | 1.010 | 99.01M | nan | — | — |
| 1,000,000 | 25.003 | 40.00M | 16.982 | 58.89M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.199 ms**; native kernel **1.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.339 | 0.291 | 3.44M | nan | — | — |
| 100,000 | 10 | 2.419 | 1.340 | 7.46M | nan | — | — |
| 100,000 | 1,000 | 14.982 | 12.110 | 82.57M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 64.41M | 80.41M | 1.00× | 2.14M | 2.07M | 1.00× | — |
| 2 | 83.66M | 151.63M | 1.89× | 2.28M | 2.35M | 1.14× | — |
| 4 | 108.11M | 225.44M | 2.80× | 2.14M | 2.22M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
