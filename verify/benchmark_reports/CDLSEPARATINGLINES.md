# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.65M | 0.005 | 194.76M | 0.033 | 4.90× | 6.46× |
| 10,000 | 0.068 | 147.44M | 0.064 | 157.14M | 0.122 | 1.81× | 1.92× |
| 100,000 | 0.729 | 137.11M | 0.715 | 139.89M | 0.963 | 1.32× | 1.35× |
| 1,000,000 | 7.762 | 128.83M | 7.519 | 132.99M | 9.598 | 1.24× | 1.28× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.724 ms**; native kernel **0.711 ms**; TA-Lib 0.959 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.325 | 0.271 | 3.69M | 944.499 | 3485.14× | 102.01× |
| 100,000 | 10 | 2.469 | 1.325 | 7.55M | 966.656 | 729.70× | 20.61× |
| 100,000 | 1,000 | 24.952 | 23.397 | 42.74M | 958.605 | 40.97× | 1.41× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 101.77M | 113.18M | 1.00× | 2.48M | 2.09M | 1.00× | 88.31M |
| 2 | 168.62M | 224.10M | 1.98× | 2.22M | 2.69M | 1.29× | 82.34M |
| 4 | 380.69M | 404.12M | 3.57× | 2.34M | 2.58M | 1.23× | 86.91M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
