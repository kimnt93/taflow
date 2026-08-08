# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.19M | 0.006 | 162.08M | 0.032 | 0.68× | 5.18× |
| 10,000 | 0.493 | 20.27M | 0.053 | 187.38M | 0.073 | 0.15× | 1.38× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.009 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.517 | 0.181 | 5.53M | 33.928 | 187.79× | 149.20× |
| 1,500 | 10 | 1.662 | 0.686 | 14.57M | 33.579 | 48.91× | 40.65× |
| 1,500 | 100 | 6.882 | 2.517 | 39.73M | 35.629 | 14.16× | 10.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
