# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.56M | 0.038 | 26.59M | 0.040 | 1.06× | 1.06× |
| 10,000 | 0.402 | 24.86M | 0.443 | 22.57M | 0.113 | 0.28× | 0.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.059 ms**; native kernel **0.055 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.599 | 0.503 | 1.99M | 42.192 | 83.85× | 65.43× |
| 1,500 | 10 | 3.074 | 1.510 | 6.62M | 41.433 | 27.43× | 23.29× |
| 1,500 | 100 | 9.175 | 6.184 | 16.17M | 43.049 | 6.96× | 5.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
