# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.72M | 0.006 | 157.73M | 0.038 | 0.79× | 6.03× |
| 10,000 | 0.493 | 20.30M | 0.055 | 180.35M | 0.091 | 0.18× | 1.64× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.073 ms**; native kernel **0.009 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.302 | 0.179 | 5.58M | 42.584 | 237.57× | 181.44× |
| 1,500 | 10 | 1.758 | 0.736 | 13.59M | 43.311 | 58.85× | 46.00× |
| 1,500 | 100 | 8.376 | 2.661 | 37.58M | 42.613 | 16.01× | 11.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
