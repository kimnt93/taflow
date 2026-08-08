# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.03M | 0.006 | 173.21M | 0.037 | 5.12× | 6.47× |
| 10,000 | 0.061 | 164.19M | 0.053 | 189.94M | 0.082 | 1.35× | 1.56× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.295 | 0.219 | 4.56M | 38.384 | 175.18× | 142.02× |
| 1,500 | 10 | 1.732 | 0.815 | 12.27M | 40.995 | 50.32× | 37.54× |
| 1,500 | 100 | 5.658 | 3.240 | 30.86M | 40.428 | 12.48× | 9.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
