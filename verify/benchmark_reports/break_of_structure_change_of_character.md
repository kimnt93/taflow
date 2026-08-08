# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.75M | 0.085 | 11.78M | nan | — | — |
| 10,000 | 0.706 | 14.17M | 0.685 | 14.59M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.107 ms**; native kernel **0.109 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.477 | 0.368 | 2.72M | nan | — | — |
| 1,500 | 10 | 2.812 | 1.716 | 5.83M | nan | — | — |
| 1,500 | 100 | 10.385 | 9.061 | 11.04M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
