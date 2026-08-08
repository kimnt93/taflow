# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.06M | 0.009 | 110.27M | nan | — | — |
| 10,000 | 0.499 | 20.03M | 0.083 | 120.21M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.075 ms**; native kernel **0.014 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.363 | 0.209 | 4.78M | nan | — | — |
| 1,500 | 10 | 1.695 | 0.729 | 13.72M | nan | — | — |
| 1,500 | 100 | 6.739 | 2.769 | 36.11M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
