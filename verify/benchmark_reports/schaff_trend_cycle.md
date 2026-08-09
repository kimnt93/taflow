# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.42M | 0.063 | 15.82M | nan | — | — |
| 10,000 | 0.679 | 14.74M | 0.634 | 15.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.100 ms**; native kernel **0.092 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.350 | 0.279 | 3.59M | nan | — | — |
| 1,500 | 10 | 1.588 | 1.124 | 8.90M | nan | — | — |
| 1,500 | 100 | 7.804 | 7.169 | 13.95M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.20M | 5.88M | 1.00× | 680.84K | 966.74K | 1.00× | — |
| 2 | 10.12M | 12.48M | 2.12× | 1.21M | 1.01M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
