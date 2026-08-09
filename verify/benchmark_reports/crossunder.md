# Crossunder benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.72M | 0.008 | 125.62M | nan | — | — |
| 10,000 | 0.041 | 240.99M | 0.036 | 276.65M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.273 | 0.233 | 4.30M | nan | — | — |
| 1,500 | 10 | 1.817 | 0.764 | 13.09M | nan | — | — |
| 1,500 | 100 | 2.930 | 1.742 | 57.42M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 16.01M | 16.32M | 1.00× | 1.38M | 954.20K | 1.00× | — |
| 2 | 13.58M | 19.42M | 1.19× | 1.23M | 1.41M | 1.48× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
