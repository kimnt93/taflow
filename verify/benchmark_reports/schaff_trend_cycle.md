# SchaffTrendCycle benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.91M | 0.083 | 12.03M | nan | — | — |
| 10,000 | 0.919 | 10.89M | 0.932 | 10.73M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.137 ms**; native kernel **0.136 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.379 | 0.605 | 1.65M | nan | — | — |
| 1,500 | 10 | 1.803 | 1.221 | 8.19M | nan | — | — |
| 1,500 | 100 | 9.011 | 26.735 | 3.74M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
