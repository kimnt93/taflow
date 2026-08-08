# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.48M | 0.016 | 63.73M | 0.033 | 1.87× | 2.11× |
| 10,000 | 0.225 | 44.42M | 0.227 | 44.03M | 0.112 | 0.50× | 0.50× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.025 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.426 | 0.339 | 2.95M | 36.782 | 108.46× | 84.69× |
| 1,500 | 10 | 4.672 | 1.381 | 7.24M | 35.154 | 25.45× | 20.72× |
| 1,500 | 100 | 8.219 | 5.214 | 19.18M | 36.077 | 6.92× | 5.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
