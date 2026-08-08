# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.52M | 0.057 | 17.69M | nan | — | — |
| 10,000 | 0.593 | 16.87M | 0.573 | 17.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.090 ms**; native kernel **0.087 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.638 | 0.485 | 2.06M | nan | — | — |
| 1,500 | 10 | 3.554 | 2.161 | 4.63M | nan | — | — |
| 1,500 | 100 | 12.850 | 10.632 | 9.41M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
