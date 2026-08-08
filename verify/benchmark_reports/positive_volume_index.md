# PositiveVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.71M | 0.005 | 183.90M | nan | — | — |
| 10,000 | 0.067 | 149.62M | 0.061 | 163.76M | nan | — | — |
| 100,000 | 0.596 | 167.68M | 0.568 | 175.96M | nan | — | — |
| 1,000,000 | 7.179 | 139.30M | 6.104 | 163.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.621 ms**; native kernel **0.573 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.261 | 0.212 | 4.73M | nan | — | — |
| 100,000 | 10 | 1.548 | 0.810 | 12.35M | nan | — | — |
| 100,000 | 1,000 | 9.188 | 8.759 | 114.17M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 128.06M | 139.01M | 1.00× | 2.92M | 3.01M | 1.00× | — |
| 2 | 252.23M | 258.19M | 1.86× | 3.34M | 3.48M | 1.16× | — |
| 4 | 214.16M | 174.96M | 1.26× | 2.88M | 2.81M | 0.93× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
