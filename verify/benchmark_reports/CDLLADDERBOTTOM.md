# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.42M | 0.020 | 49.46M | 0.034 | 1.58× | 1.69× |
| 10,000 | 0.210 | 47.73M | 0.218 | 45.90M | 0.090 | 0.43× | 0.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.033 ms**; native kernel **0.029 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.387 | 0.301 | 3.32M | 35.482 | 117.86× | 97.17× |
| 1,500 | 10 | 2.725 | 1.282 | 7.80M | 35.707 | 27.85× | 22.07× |
| 1,500 | 100 | 14.070 | 4.704 | 21.26M | 37.569 | 7.99× | 6.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
