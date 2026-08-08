# AverageDailyDollarValue benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.94M | 0.007 | 153.70M | nan | — | — |
| 10,000 | 0.056 | 179.52M | 0.053 | 189.16M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.338 | 0.241 | 4.16M | nan | — | — |
| 1,500 | 10 | 1.760 | 0.841 | 11.90M | nan | — | — |
| 1,500 | 100 | 3.857 | 2.536 | 39.43M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
