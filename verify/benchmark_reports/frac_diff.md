# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.83M | 0.072 | 13.97M | nan | — | — |
| 10,000 | 7.193 | 1.39M | 7.337 | 1.36M | nan | — | — |
| 100,000 | 79.204 | 1.26M | 79.672 | 1.26M | nan | — | — |
| 1,000,000 | 796.261 | 1.26M | 794.021 | 1.26M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **78.154 ms**; native kernel **79.191 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 1.016 | 0.933 | 1.07M | nan | — | — |
| 100,000 | 10 | 9.150 | 8.363 | 1.20M | nan | — | — |
| 100,000 | 1,000 | 790.850 | 828.931 | 1.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.26M | 1.25M | 1.00× | 818.18K | 781.94K | 1.00× | — |
| 2 | 2.41M | 2.43M | 1.95× | 805.10K | 841.81K | 1.08× | — |
| 4 | 4.66M | 4.71M | 3.77× | 841.16K | 851.33K | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
