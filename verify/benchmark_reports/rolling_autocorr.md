# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 18.99M | 0.052 | 19.09M | nan | — | — |
| 10,000 | 0.531 | 18.83M | 0.510 | 19.62M | nan | — | — |
| 100,000 | 5.184 | 19.29M | 5.150 | 19.42M | nan | — | — |
| 1,000,000 | 51.057 | 19.59M | 51.436 | 19.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.018 ms**; native kernel **5.098 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.246 | 0.197 | 5.08M | nan | — | — |
| 100,000 | 10 | 1.655 | 0.942 | 10.62M | nan | — | — |
| 100,000 | 1,000 | 51.521 | 50.579 | 19.77M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18.03M | 18.47M | 1.00× | 2.79M | 3.25M | 1.00× | — |
| 2 | 34.39M | 35.01M | 1.90× | 2.89M | 2.67M | 0.82× | — |
| 4 | 66.05M | 67.58M | 3.66× | 2.60M | 2.74M | 0.84× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
