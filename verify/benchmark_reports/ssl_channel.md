# SmoothedTrendChannel benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.82M | 0.014 | 69.69M | nan | — | — |
| 10,000 | 0.135 | 73.92M | 0.148 | 67.65M | nan | — | — |
| 100,000 | 1.385 | 72.22M | 1.465 | 68.25M | nan | — | — |
| 1,000,000 | 15.124 | 66.12M | 14.609 | 68.45M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.380 ms**; native kernel **1.722 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.467 | 0.290 | 3.45M | nan | — | — |
| 100,000 | 10 | 1.615 | 1.133 | 8.83M | nan | — | — |
| 100,000 | 1,000 | 20.412 | 20.807 | 48.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 55.30M | 58.26M | 1.00× | 1.64M | 2.37M | 1.00× | — |
| 2 | 53.06M | 65.34M | 1.12× | 1.80M | 2.17M | 0.92× | — |
| 4 | 56.79M | 64.41M | 1.11× | 1.79M | 2.34M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
