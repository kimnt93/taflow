# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.81M | 0.009 | 107.58M | 0.039 | 3.49× | 4.18× |
| 10,000 | 0.090 | 111.25M | 0.086 | 116.57M | 0.178 | 1.98× | 2.07× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.364 | 0.277 | 3.62M | 45.249 | 163.58× | 103.44× |
| 1,500 | 10 | 2.579 | 1.263 | 7.92M | 47.456 | 37.57× | 22.48× |
| 1,500 | 100 | 5.832 | 3.522 | 28.40M | 46.504 | 13.20× | 8.64× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.76M | 13.31M | 1.00× | 908.28K | 1.35M | 1.00× | 7.74M |
| 2 | 14.22M | 14.69M | 1.10× | 1.23M | 1.16M | 0.86× | 8.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
