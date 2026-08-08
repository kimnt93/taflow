# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.11M | 0.009 | 108.72M | nan | — | — |
| 10,000 | 0.088 | 114.14M | 0.085 | 118.14M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.299 | 0.183 | 5.45M | nan | — | — |
| 1,500 | 10 | 0.861 | 0.571 | 17.53M | nan | — | — |
| 1,500 | 100 | 2.679 | 2.318 | 43.14M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
