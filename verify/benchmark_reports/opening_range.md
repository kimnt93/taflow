# OpeningRange benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.33M | 0.008 | 119.72M | nan | — | — |
| 10,000 | 0.075 | 133.16M | 0.067 | 149.86M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.505 | 0.487 | 2.05M | nan | — | — |
| 1,500 | 10 | 2.003 | 1.267 | 7.89M | nan | — | — |
| 1,500 | 100 | 4.027 | 3.092 | 32.34M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
