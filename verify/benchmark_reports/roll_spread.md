# RollSpread benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.18M | 0.043 | 23.37M | nan | — | — |
| 10,000 | 0.423 | 23.64M | 0.440 | 22.71M | nan | — | — |
| 100,000 | 4.385 | 22.80M | 4.286 | 23.33M | nan | — | — |
| 1,000,000 | 43.709 | 22.88M | 43.453 | 23.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.389 ms**; native kernel **4.367 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.254 | 0.212 | 4.71M | nan | — | — |
| 100,000 | 10 | 1.468 | 1.057 | 9.46M | nan | — | — |
| 100,000 | 1,000 | 48.657 | 45.181 | 22.13M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.20M | 21.52M | 1.00× | 2.66M | 2.76M | 1.00× | — |
| 2 | 41.18M | 41.89M | 1.95× | 2.73M | 2.93M | 1.06× | — |
| 4 | 73.69M | 74.17M | 3.45× | 2.77M | 2.83M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
