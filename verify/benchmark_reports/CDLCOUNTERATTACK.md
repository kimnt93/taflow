# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 31.87M | 0.031 | 32.45M | 0.033 | 1.07× | 1.09× |
| 10,000 | 0.329 | 30.44M | 0.322 | 31.09M | 0.138 | 0.42× | 0.43× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.048 ms**; native kernel **0.046 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.674 | 0.330 | 3.03M | 39.870 | 120.66× | 83.99× |
| 1,500 | 10 | 2.976 | 1.477 | 6.77M | 39.693 | 26.87× | 18.81× |
| 1,500 | 100 | 8.312 | 5.682 | 17.60M | 40.425 | 7.11× | 4.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
