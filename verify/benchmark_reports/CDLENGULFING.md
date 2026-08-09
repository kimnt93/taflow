# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.17M | 0.007 | 140.75M | 0.035 | 4.12× | 4.99× |
| 10,000 | 0.067 | 148.41M | 0.064 | 156.29M | 0.096 | 1.43× | 1.50× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.010 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.737 | 0.656 | 1.52M | 36.365 | 55.39× | 48.21× |
| 1,500 | 10 | 2.684 | 1.446 | 6.92M | 36.304 | 25.11× | 21.35× |
| 1,500 | 100 | 5.267 | 2.916 | 34.29M | 36.485 | 12.51× | 10.75× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.64M | 15.23M | 1.00× | 939.08K | 1.02M | 1.00× | 8.22M |
| 2 | 12.53M | 14.13M | 0.93× | 907.22K | 1.06M | 1.04× | 7.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
