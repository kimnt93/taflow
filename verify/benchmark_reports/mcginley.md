# McGinleyDynamic benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.02M | 0.014 | 73.48M | nan | — | — |
| 10,000 | 0.129 | 77.22M | 0.124 | 80.96M | nan | — | — |
| 100,000 | 1.258 | 79.46M | 1.221 | 81.90M | nan | — | — |
| 1,000,000 | 13.001 | 76.92M | 12.275 | 81.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.219 ms**; native kernel **1.233 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.208 | 0.140 | 7.13M | nan | — | — |
| 100,000 | 10 | 0.958 | 0.596 | 16.78M | nan | — | — |
| 100,000 | 1,000 | 13.974 | 13.291 | 75.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 67.43M | 71.01M | 1.00× | 3.08M | 3.16M | 1.00× | — |
| 2 | 127.02M | 141.31M | 1.99× | 3.33M | 3.42M | 1.08× | — |
| 4 | 217.40M | 260.19M | 3.66× | 3.44M | 3.44M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
