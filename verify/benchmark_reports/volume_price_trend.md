# VolumePriceTrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.00M | 0.006 | 162.36M | nan | — | — |
| 10,000 | 0.032 | 310.76M | 0.029 | 339.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.270 | 0.197 | 5.08M | nan | — | — |
| 1,500 | 10 | 1.472 | 0.783 | 12.77M | nan | — | — |
| 1,500 | 100 | 2.686 | 1.715 | 58.31M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.71M | 8.15M | 1.00× | 1.03M | 1.24M | 1.00× | — |
| 2 | 17.33M | 21.35M | 2.62× | 1.58M | 1.58M | 1.27× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
