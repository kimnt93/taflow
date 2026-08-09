# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.76M | 0.021 | 48.25M | 0.052 | 2.23× | 2.52× |
| 10,000 | 0.246 | 40.63M | 0.220 | 45.56M | 0.200 | 0.81× | 0.91× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.031 ms**; TA-Lib 0.059 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.297 | 0.233 | 4.29M | 58.997 | 252.91× | 184.46× |
| 1,500 | 10 | 1.231 | 1.036 | 9.65M | 61.504 | 59.36× | 40.66× |
| 1,500 | 100 | 7.134 | 7.219 | 13.85M | 60.586 | 8.39× | 5.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.07M | 10.34M | 1.00× | 1.08M | 1.11M | 1.00× | 7.34M |
| 2 | 11.98M | 20.06M | 1.94× | 1.32M | 1.02M | 0.91× | 8.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
