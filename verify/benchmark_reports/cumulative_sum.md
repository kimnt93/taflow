# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.11M | 0.003 | 294.03M | nan | — | — |
| 10,000 | 0.029 | 342.61M | 0.026 | 384.20M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.263 | 0.172 | 5.80M | nan | — | — |
| 1,500 | 10 | 0.989 | 0.510 | 19.60M | nan | — | — |
| 1,500 | 100 | 2.527 | 1.717 | 58.24M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
