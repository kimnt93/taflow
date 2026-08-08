# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.96M | 0.019 | 53.54M | 0.034 | 1.68× | 1.83× |
| 10,000 | 0.213 | 46.94M | 0.203 | 49.17M | 0.111 | 0.52× | 0.55× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.029 ms**; native kernel **0.027 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.373 | 0.483 | 2.07M | 36.238 | 75.07× | 66.11× |
| 1,500 | 10 | 2.831 | 1.353 | 7.39M | 35.856 | 26.51× | 23.00× |
| 1,500 | 100 | 7.465 | 4.480 | 22.32M | 35.436 | 7.91× | 7.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
