# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.11M | 0.010 | 96.56M | nan | — | — |
| 10,000 | 0.096 | 104.33M | 0.095 | 105.77M | nan | — | — |
| 100,000 | 0.920 | 108.66M | 0.922 | 108.50M | nan | — | — |
| 1,000,000 | 9.731 | 102.77M | 9.353 | 106.91M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.953 ms**; native kernel **0.920 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.168 | 5.97M | nan | — | — |
| 100,000 | 10 | 1.061 | 0.638 | 15.68M | nan | — | — |
| 100,000 | 1,000 | 11.646 | 10.696 | 93.49M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 93.95M | 102.21M | 1.00× | 3.07M | 2.74M | 1.00× | — |
| 2 | 149.94M | 200.85M | 1.97× | 3.43M | 3.70M | 1.35× | — |
| 4 | 285.22M | 376.77M | 3.69× | 3.38M | 3.29M | 1.20× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
