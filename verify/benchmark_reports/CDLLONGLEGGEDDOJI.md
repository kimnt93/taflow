# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.84M | 0.008 | 125.00M | 0.035 | 3.45× | 4.36× |
| 10,000 | 0.086 | 116.24M | 0.082 | 122.58M | 0.093 | 1.08× | 1.14× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.362 | 0.271 | 3.69M | 38.321 | 141.33× | 104.27× |
| 1,500 | 10 | 2.648 | 1.206 | 8.29M | 37.694 | 31.27× | 25.04× |
| 1,500 | 100 | 6.147 | 3.558 | 28.11M | 38.875 | 10.93× | 8.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
