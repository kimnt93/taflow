# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.46M | 0.015 | 67.89M | 0.082 | 5.44× | 5.56× |
| 10,000 | 0.132 | 75.75M | 0.128 | 78.31M | 0.593 | 4.49× | 4.64× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.021 ms**; TA-Lib 0.115 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.268 | 0.203 | 4.93M | 113.723 | 560.91× | 165.19× |
| 1,500 | 10 | 0.977 | 0.860 | 11.63M | 115.180 | 134.00× | 35.85× |
| 1,500 | 100 | 3.741 | 3.005 | 33.28M | 120.774 | 40.20× | 11.01× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.29M | 15.20M | 1.00× | 1.39M | 1.37M | 1.00× | 5.23M |
| 2 | 19.74M | 20.91M | 1.38× | 1.52M | 1.54M | 1.12× | 6.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
