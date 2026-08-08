# ChaikinMoneyFlow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.01M | 0.011 | 91.22M | nan | — | — |
| 10,000 | 0.100 | 100.41M | 0.094 | 105.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.016 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.435 | 0.316 | 3.16M | nan | — | — |
| 1,500 | 10 | 2.756 | 1.915 | 5.22M | nan | — | — |
| 1,500 | 100 | 5.096 | 3.342 | 29.92M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
