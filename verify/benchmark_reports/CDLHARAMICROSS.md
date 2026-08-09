# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.68M | 0.009 | 110.54M | 0.038 | 3.44× | 4.15× |
| 10,000 | 0.109 | 91.98M | 0.091 | 109.51M | 0.143 | 1.32× | 1.57× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.356 | 0.279 | 3.58M | 41.975 | 150.44× | 103.39× |
| 1,500 | 10 | 2.611 | 1.301 | 7.68M | 43.289 | 33.26× | 23.66× |
| 1,500 | 100 | 5.695 | 3.151 | 31.74M | 42.809 | 13.59× | 9.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.42M | 10.60M | 1.00× | 955.58K | 1.30M | 1.00× | 8.54M |
| 2 | 12.94M | 17.98M | 1.70× | 1.26M | 1.22M | 0.94× | 9.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
