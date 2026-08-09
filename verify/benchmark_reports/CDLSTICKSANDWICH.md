# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.45M | 0.007 | 140.85M | 0.032 | 3.64× | 4.52× |
| 10,000 | 0.043 | 234.32M | 0.039 | 254.36M | 0.087 | 2.04× | 2.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.348 | 0.292 | 3.42M | 34.375 | 117.57× | 97.09× |
| 1,500 | 10 | 2.541 | 1.261 | 7.93M | 34.485 | 27.34× | 22.41× |
| 1,500 | 100 | 5.357 | 3.152 | 31.73M | 46.450 | 14.74× | 10.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.94M | 13.23M | 1.00× | 1.04M | 1.10M | 1.00× | 7.98M |
| 2 | 12.63M | 14.29M | 1.08× | 928.15K | 980.13K | 0.89× | 7.54M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
