# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.11M | 0.044 | 22.87M | 0.042 | 0.94× | 0.97× |
| 10,000 | 0.441 | 22.66M | 0.444 | 22.54M | 0.181 | 0.41× | 0.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.065 ms**; TA-Lib 0.047 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.429 | 0.345 | 2.90M | 49.049 | 142.34× | 80.87× |
| 1,500 | 10 | 3.111 | 1.646 | 6.07M | 45.554 | 27.67× | 17.29× |
| 1,500 | 100 | 10.078 | 6.964 | 14.36M | 47.790 | 6.86× | 4.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
