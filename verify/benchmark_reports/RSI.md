# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.15M | 0.008 | 126.80M | 0.038 | 4.36× | 4.81× |
| 10,000 | 0.064 | 155.30M | 0.060 | 166.84M | 0.083 | 1.29× | 1.39× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.011 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.251 | 0.172 | 5.82M | 39.557 | 230.31× | 178.50× |
| 1,500 | 10 | 1.015 | 0.749 | 13.35M | 38.266 | 51.07× | 41.61× |
| 1,500 | 100 | 3.033 | 2.178 | 45.91M | 38.765 | 17.80× | 15.15× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.68M | 12.90M | 1.00× | 1.42M | 1.49M | 1.00× | 9.04M |
| 2 | 19.49M | 21.45M | 1.66× | 1.50M | 1.68M | 1.12× | 9.76M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
