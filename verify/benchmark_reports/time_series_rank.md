# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.03M | 0.015 | 68.60M | nan | — | — |
| 10,000 | 0.143 | 70.00M | 0.137 | 72.79M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.022 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.354 | 0.213 | 4.71M | nan | — | — |
| 1,500 | 10 | 1.191 | 0.692 | 14.46M | nan | — | — |
| 1,500 | 100 | 3.760 | 3.049 | 32.80M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
