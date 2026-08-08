# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.91M | 0.021 | 46.76M | nan | — | — |
| 10,000 | 0.321 | 31.10M | 0.283 | 35.37M | nan | — | — |
| 100,000 | 3.413 | 29.30M | 2.807 | 35.62M | nan | — | — |
| 1,000,000 | 59.433 | 16.83M | 49.699 | 20.12M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.704 ms**; native kernel **2.745 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.321 | 0.254 | 3.94M | nan | — | — |
| 100,000 | 10 | 0.896 | 0.688 | 14.54M | nan | — | — |
| 100,000 | 1,000 | 28.408 | 27.289 | 36.64M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 29.65M | 34.61M | 1.00× | 1.94M | 2.24M | 1.00× | — |
| 2 | 26.62M | 33.09M | 0.96× | 1.92M | 2.23M | 1.00× | — |
| 4 | 28.14M | 31.51M | 0.91× | 1.98M | 1.93M | 0.86× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
