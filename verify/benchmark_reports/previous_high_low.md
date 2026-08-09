# PreviousHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.73M | 0.016 | 64.20M | nan | — | — |
| 10,000 | 0.116 | 86.37M | 0.103 | 97.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.024 ms**; native kernel **0.020 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.533 | 0.436 | 2.29M | nan | — | — |
| 1,500 | 10 | 2.115 | 1.164 | 8.59M | nan | — | — |
| 1,500 | 100 | 9.142 | 2.752 | 36.34M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.08M | 13.62M | 1.00× | 956.18K | 874.91K | 1.00× | — |
| 2 | 14.24M | 19.90M | 1.46× | 1.07M | 1.05M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
