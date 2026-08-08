# KeltnerChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.27M | 0.010 | 99.29M | nan | — | — |
| 10,000 | 0.094 | 106.64M | 0.082 | 122.62M | nan | — | — |
| 100,000 | 0.861 | 116.16M | 0.756 | 132.28M | nan | — | — |
| 1,000,000 | 19.811 | 50.48M | 8.716 | 114.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.815 ms**; native kernel **0.774 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.365 | 0.295 | 3.39M | nan | — | — |
| 100,000 | 10 | 2.100 | 1.179 | 8.48M | nan | — | — |
| 100,000 | 1,000 | 27.536 | 13.235 | 75.56M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 61.37M | 65.01M | 1.00× | 2.24M | 2.07M | 1.00× | — |
| 2 | 100.83M | 166.87M | 2.57× | 2.19M | 2.42M | 1.17× | — |
| 4 | 140.61M | 176.04M | 2.71× | 1.79M | 2.51M | 1.21× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
