# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.98M | 0.002 | 555.99M | nan | — | — |
| 10,000 | 0.414 | 24.15M | 0.010 | 964.32M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.064 ms**; native kernel **0.002 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.441 | 0.181 | 5.52M | nan | — | — |
| 1,500 | 10 | 1.694 | 0.650 | 15.39M | nan | — | — |
| 1,500 | 100 | 6.476 | 2.071 | 48.29M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
