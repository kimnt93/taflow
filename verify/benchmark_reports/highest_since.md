# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.42M | 0.004 | 224.73M | nan | — | — |
| 10,000 | 0.040 | 251.75M | 0.036 | 275.14M | nan | — | — |
| 100,000 | 0.366 | 272.98M | 0.331 | 302.25M | nan | — | — |
| 1,000,000 | 3.858 | 259.21M | 3.587 | 278.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.371 ms**; native kernel **0.333 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.269 | 3.71M | nan | — | — |
| 100,000 | 10 | 1.190 | 0.731 | 13.68M | nan | — | — |
| 100,000 | 1,000 | 5.633 | 4.680 | 213.67M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 192.71M | 218.68M | 1.00× | 2.02M | 2.37M | 1.00× | — |
| 2 | 330.39M | 421.90M | 1.93× | 2.37M | 2.46M | 1.04× | — |
| 4 | 486.69M | 747.65M | 3.42× | 2.35M | 2.37M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
